"""XPM sprites with transparency, and text. Esc or Q quits."""
from pathlib import Path

import malibx

W, H = 640, 480
HERE = Path(__file__).parent

mlx = malibx.init()
win = malibx.new_window(mlx, W, H, "malibx sprites")

# From a file: "." pixels are `None`, so the background shows through.
ghost, gw, gh = malibx.xpm_file_to_image(mlx, HERE / "assets" / "ghost.xpm")

# From data in code: the strings of an XPM file, as a list.
dot, _, _ = malibx.xpm_to_image(mlx, [
    "4 4 2 1",
    ". c None",
    "o c #FFD700",
    ".oo.",
    "oooo",
    "oooo",
    ".oo.",
])

# A blue floor so the transparency is visible.
floor = malibx.new_image(mlx, W, 120)
buf, _, size_line, _ = malibx.get_data_addr(floor)
buf[:] = (0x203060).to_bytes(4, "little") * (len(buf) // 4)

state = {"x": 0, "score": 0}


def frame(st):
    malibx.clear_window(mlx, win)
    malibx.put_image_to_window(mlx, win, floor, 0, 300)
    for i in range(20):
        malibx.put_image_to_window(mlx, win, dot, 40 + i * 30, 360)
    malibx.put_image_to_window(mlx, win, ghost, st["x"], 352)
    malibx.string_put(mlx, win, 20, 30, 0xFFFFFF, f"SCORE {st['score']:06d}")
    malibx.string_put(mlx, win, 20, 50, 0xFFD700, "Hello from malibx!")
    st["x"] = (st["x"] + 2) % W
    st["score"] += 10


def key(keysym, st):
    if keysym in (0xFF1B, ord("q")):
        malibx.loop_end(mlx)


malibx.hook(win, 2, 1 << 0, key, state)
malibx.loop_hook(mlx, frame, state)
malibx.loop(mlx)
