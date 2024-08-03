struct CameraUniform {
    view_proj: mat4x4<f32>,
};

struct ConfigUniform {
    apply_camera: u32,
    _padding: u32,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(1)
var<uniform> config: ConfigUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
    @location(6) tex_index: u32,
    @location(7) atlas_index: u32,
}

struct TextureAtlas {
    atlas_width: u32,
    atlas_height: u32,
    tile_width: u32,
    tile_height: u32,
    texture_width: u32,
    texture_height: u32,
    _padding: u32,
    _padding2: u32,
}

@group(0) @binding(1) var<uniform> atlas_infos: array<TextureAtlas, 16>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) atlas_index: u32,
}

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let atlas = atlas_infos[instance.atlas_index];


    var position: vec4<f32>;

    if (config.apply_camera != 0u) {
        position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    } else {
        position = model_matrix * vec4<f32>(model.position, 1.0);
    }

    out.clip_position = position;

    let tex_index = instance.tex_index;
    let num_rows = atlas.atlas_height / atlas.tile_height;
    let num_cols = atlas.atlas_width / atlas.tile_width;

    let x_offset = tex_index % num_cols;
    let y_offset = tex_index / num_cols;

    let x_pos = (f32(x_offset) + model.tex_coords.x) / f32(num_cols);
    let y_pos = (f32(y_offset) + model.tex_coords.y) / f32(num_rows);

    out.tex_coords = vec2<f32>(
        x_pos * (f32(atlas.atlas_width) / f32(atlas.texture_width)),
        y_pos * (f32(atlas.atlas_height) / f32(atlas.texture_height))
    );
    out.atlas_index = instance.atlas_index;

    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d_array<f32>; // Declare a 2D texture array
@group(0) @binding(2)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the texture array at the specified 2D coordinates and layer index
    return textureSample(t_diffuse, s_diffuse, in.tex_coords, in.atlas_index);
}
