"""A big XPM skull floating up and down, with glowing eyes that look around.

Esc or Q quits.
"""
import math
from pathlib import Path

import malibx

W, H = 640, 480
SCALE = 8       # 48 px skull -> 384 px
BOB = 14        # pixels up and down
PERIOD = 150    # frames per bob (2.5 s at 60 fps)
HERE = Path(__file__).parent

SOCKET = 0x0B0908  # the eye sockets' color in skull.xpm
CLEAR = (0xFF000000).to_bytes(4, "little")  # alpha 0xFF = transparent

# Eye glow, from the pupil outwards: (max distance in skull pixels, color).
GLOW = [(1.3, 0xF6E0FF), (2.6, 0xC860FF), (4.2, 0x8424D8), (99, 0x3C0C70)]
RIM_GLOW = [0x4A2466, 0x5C2C82, 0x70349E]   # bone next to the eyes, per pulse
PULSE = [0.85, 1.0, 1.2]                    # glow size, per pulse
PULSE_PERIOD = 90

# Where the eyes look (pupil offset in skull pixels) and for how many frames.
LOOK = [(0, 50), (-2, 6), (-3, 70), (-2, 6), (0, 40), (2, 6), (3, 70), (2, 6)]

mlx = malibx.init()
win = malibx.new_window(mlx, W, H, "malibx sprites")


def scale(img, w, h, s):
    """A new image s times bigger, built by hand (MLX has no scaling)."""
    src, _, src_line, _ = malibx.get_data_addr(img)
    big = malibx.new_image(mlx, w * s, h * s)
    dst, _, dst_line, _ = malibx.get_data_addr(big)
    for y in range(h):
        row = src[y * src_line:(y + 1) * src_line]
        wide = b"".join(bytes(row[x * 4:x * 4 + 4]) * s for x in range(w))
        for i in range(s):
            start = (y * s + i) * dst_line
            dst[start:start + dst_line] = wide
    return big


def pixels(img, w, h):
    """{(x, y): 0xRRGGBB} for every pixel of an image."""
    buf, _, line, _ = malibx.get_data_addr(img)
    return {
        (x, y): int.from_bytes(buf[y * line + x * 4:y * line + x * 4 + 3],
                               "little")
        for y in range(h) for x in range(w)
    }


# "." pixels are `None`, so the background shows through.
SKULL = HERE.parent / "assets" / "skull.xpm"
skull, sw, sh = malibx.xpm_file_to_image(mlx, SKULL)
big = scale(skull, sw, sh, SCALE)
bw, bh = sw * SCALE, sh * SCALE

# Find the eye sockets in the skull itself, and the bone right around them.
px = pixels(skull, sw, sh)
sockets = [p for p, c in px.items() if c == SOCKET]
taken = set(sockets)
rims = sorted({(x + dx, y + dy) for x, y in sockets
               for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))} - taken)
x0 = min(x for x, _ in sockets + rims)
x1 = max(x for x, _ in sockets + rims)
y0 = min(y for _, y in sockets + rims)
y1 = max(y for _, y in sockets + rims)
centers = []
for side in (lambda x: x < sw // 2, lambda x: x >= sw // 2):
    pts = [(x, y) for x, y in sockets if side(x)]
    centers.append((sum(x for x, _ in pts) / len(pts),
                    sum(y for _, y in pts) / len(pts)))


def eyes(look, pulse):
    """The glowing eyes (and lit rims) as one transparent overlay image."""
    ew, eh = x1 - x0 + 1, y1 - y0 + 1
    img = malibx.new_image(mlx, ew, eh)
    buf, _, line, _ = malibx.get_data_addr(img)
    buf[:] = CLEAR * (ew * eh)
    for x, y in sockets:
        cx, cy = min(centers, key=lambda c: abs(c[0] - x))
        d = math.hypot(x - (cx + look), (y - cy) * 1.2) / PULSE[pulse]
        color = next(c for r, c in GLOW if d <= r)
        i = (y - y0) * line + (x - x0) * 4
        buf[i:i + 4] = color.to_bytes(4, "little")
    for x, y in rims:
        i = (y - y0) * line + (x - x0) * 4
        buf[i:i + 4] = RIM_GLOW[pulse].to_bytes(4, "little")
    return scale(img, ew, eh, SCALE)


overlays = {(look, pulse): eyes(look, pulse)
            for look in {lk for lk, _ in LOOK} for pulse in range(len(PULSE))}
look_at = [look for look, frames in LOOK for _ in range(frames)]

# A dark background so the transparency is visible.
bg = malibx.new_image(mlx, W, H)
buf, _, _, _ = malibx.get_data_addr(bg)
buf[:] = (0x1A1030).to_bytes(4, "little") * (len(buf) // 4)

state = {"frame": 0}


def frame(st):
    f = st["frame"]
    x = (W - bw) // 2
    y = (H - bh) // 2 + round(math.sin(f * 2 * math.pi / PERIOD) * BOB)
    pulse = round((math.sin(f * 2 * math.pi / PULSE_PERIOD) + 1) / 2 * 2)
    look = look_at[f % len(look_at)]
    malibx.put_image_to_window(mlx, win, bg, 0, 0)
    malibx.put_image_to_window(mlx, win, big, x, y)
    malibx.put_image_to_window(mlx, win, overlays[look, pulse],
                               x + x0 * SCALE, y + y0 * SCALE)
    malibx.string_put(mlx, win, 16, 24, 0xFFFFFF, "malibx sprites")
    malibx.string_put(mlx, win, 16, 40, 0xB040FF, f"frame {f}")
    st["frame"] += 1


def key(keysym, st):
    if keysym in (0xFF1B, ord("q")):
        malibx.loop_end(mlx)


malibx.hook(win, 2, 1 << 0, key, state)
malibx.loop_hook(mlx, frame, state)
malibx.loop(mlx)
