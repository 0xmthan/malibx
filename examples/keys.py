"""Move a square with the arrows or WASD. Esc, Q or the close button quits.

Prints every event, so you can see the X11 key codes and mouse buttons.
"""
import malibx

W, H, SIZE, SPEED = 640, 480, 32, 4

# X11 keysyms (same values as MiniLibX).
ESC, Q = 0xFF1B, ord("q")
DIRS = {
    0xFF51: (-1, 0), ord("a"): (-1, 0),  # left
    0xFF53: (1, 0), ord("d"): (1, 0),    # right
    0xFF52: (0, -1), ord("w"): (0, -1),  # up
    0xFF54: (0, 1), ord("s"): (0, 1),    # down
}

mlx = malibx.init()
win = malibx.new_window(mlx, W, H, "malibx keys")

sprite = malibx.new_image(mlx, SIZE, SIZE)
buf, _, size_line, _ = malibx.get_data_addr(sprite)
yellow = (0xFFD700).to_bytes(4, "little")
for row in range(SIZE):
    buf[row * size_line:(row + 1) * size_line] = yellow * SIZE

state = {"x": W // 2, "y": H // 2, "held": set()}


def key_press(key, st):
    print(f"press   0x{key:04x}")
    if key in (ESC, Q):
        malibx.loop_end(mlx)
    st["held"].add(key)


def key_release(key, st):
    print(f"release 0x{key:04x}")
    st["held"].discard(key)


def mouse_press(button, x, y, st):
    print(f"mouse   button {button} at ({x}, {y})")


def close(st):
    print("window closed")


def frame(st):
    for key in st["held"]:
        dx, dy = DIRS.get(key, (0, 0))
        st["x"] = min(max(st["x"] + dx * SPEED, 0), W - SIZE)
        st["y"] = min(max(st["y"] + dy * SPEED, 0), H - SIZE)
    malibx.clear_window(mlx, win)
    malibx.put_image_to_window(mlx, win, sprite, st["x"], st["y"])


malibx.hook(win, 2, 1 << 0, key_press, state)    # KeyPress
malibx.hook(win, 3, 1 << 1, key_release, state)  # KeyRelease
malibx.mouse_hook(win, mouse_press, state)       # ButtonPress
malibx.hook(win, 17, 0, close, state)            # DestroyNotify
malibx.loop_hook(mlx, frame, state)
malibx.loop(mlx)
