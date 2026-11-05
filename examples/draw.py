"""Colored squares drawn into an image, a pixel line, and a moving sprite."""
import malibx

W, H = 640, 480

mlx = malibx.init()
win = malibx.new_window(mlx, W, H, "malibx draw")


def fill_rect(buf, size_line, x, y, w, h, color):
    """Write a solid rectangle into an image buffer (B, G, R, A per pixel)."""
    px = color.to_bytes(4, "little")
    for row in range(y, y + h):
        start = row * size_line + x * 4
        buf[start:start + w * 4] = px * w


# Background: three squares, drawn once into an image.
bg = malibx.new_image(mlx, W, H)
buf, bpp, size_line, endian = malibx.get_data_addr(bg)
fill_rect(buf, size_line, 60, 60, 120, 120, 0xFF0000)
fill_rect(buf, size_line, 260, 60, 120, 120, 0x00FF00)
fill_rect(buf, size_line, 460, 60, 120, 120, 0x0000FF)

# Sprite: a 32x32 yellow square with a dark eye.
sprite = malibx.new_image(mlx, 32, 32)
sbuf, _, s_line, _ = malibx.get_data_addr(sprite)
fill_rect(sbuf, s_line, 0, 0, 32, 32, 0xFFD700)
fill_rect(sbuf, s_line, 18, 8, 6, 6, 0x202020)

state = {"x": -32}


def frame(param):
    malibx.clear_window(mlx, win)
    malibx.put_image_to_window(mlx, win, bg, 0, 0)
    for x in range(W):
        malibx.pixel_put(mlx, win, x, 300, 0xFFFFFF)
    malibx.put_image_to_window(mlx, win, sprite, state["x"], 330)
    state["x"] = state["x"] + 3 if state["x"] < W else -32


malibx.loop_hook(mlx, frame)
malibx.loop(mlx)
