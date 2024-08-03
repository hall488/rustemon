from PIL import Image

# Load the sprite sheet
sprite_sheet = Image.open('pokemon_front.png')

# Sprite sheet dimensions
sheet_width, sheet_height = sprite_sheet.size

# Dimensions of each sprite
sprite_width = 64
sprite_height = 64

# Number of sprites in the sheet
num_columns = sheet_width // sprite_width
num_rows = sheet_height // sprite_height

# Create a new image to store the aligned sprites
aligned_sheet = Image.new('RGBA', (sheet_width, sheet_height))

for row in range(num_rows):
    for col in range(num_columns):
        # Extract the current sprite
        left = col * sprite_width
        upper = row * sprite_height
        right = left + sprite_width
        lower = upper + sprite_height
        sprite = sprite_sheet.crop((left, upper, right, lower))

        # Find the bottom-most non-transparent pixel in the sprite
        bottom_most = 0
        for y in range(sprite_height - 1, -1, -1):
            for x in range(sprite_width):
                pixel = sprite.getpixel((x, y))
                if pixel[3] != 0:  # Check if the pixel is not transparent
                    bottom_most = max(bottom_most, y)
                    break

        # Calculate the vertical offset needed to align the sprite to the bottom
        offset = sprite_height - 1 - bottom_most

        # Create a new image with the sprite aligned to the bottom
        aligned_sprite = Image.new('RGBA', (sprite_width, sprite_height))
        aligned_sprite.paste(sprite, (0, offset))

        # Paste the aligned sprite back to the new sprite sheet
        aligned_sheet.paste(aligned_sprite, (left, upper))

# Save the new sprite sheet
aligned_sheet.save('aligned_pokemon_back.png')
