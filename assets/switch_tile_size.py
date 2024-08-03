from PIL import Image

def rearrange_sprite_sheet(input_path, output_path, tile_width, tile_height, old_cols, old_rows, new_cols, new_rows):
    # Open the original sprite sheet
    sprite_sheet = Image.open(input_path)

    # Calculate the total number of tiles
    total_tiles = old_cols * old_rows

    # Create a new image with the new dimensions
    new_sprite_sheet = Image.new("RGBA", (new_cols * tile_width, new_rows * tile_height))

    for tile_index in range(total_tiles):
        # Calculate the position in the old sprite sheet
        old_x = (tile_index % old_cols) * tile_width
        old_y = (tile_index // old_cols) * tile_height

        # Calculate the position in the new sprite sheet
        new_x = (tile_index % new_cols) * tile_width
        new_y = (tile_index // new_cols) * tile_height

        # Crop the tile from the old sprite sheet
        tile = sprite_sheet.crop((old_x, old_y, old_x + tile_width, old_y + tile_height))

        # Paste the tile into the new sprite sheet
        new_sprite_sheet.paste(tile, (new_x, new_y))

    # Save the new sprite sheet
    new_sprite_sheet.save(output_path)

# Parameters
input_path = "pokemon_party.png"
output_path = "re_pokemon_party.png"
tile_width = 32
tile_height = 32
old_cols = 25
old_rows = 7
new_cols = 16
new_rows = 10

# Rearrange the sprite sheet
rearrange_sprite_sheet(input_path, output_path, tile_width, tile_height, old_cols, old_rows, new_cols, new_rows)

