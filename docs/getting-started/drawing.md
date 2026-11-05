# Drawing

malibx has three ways to put pixels on screen:

| Method | Best for | Speed |
|---|---|---|
| [`pixel_put`](#single-pixels) | A few dots, lines, debugging | :material-speedometer-slow: one call per pixel |
| [`string_put`](#text) | Labels, scores, HUDs | :material-speedometer-medium: one call per string |
| [Images](#images) | Backgrounds, sprites, anything big | :material-speedometer: one call per image |

All three draw into the window's pixel buffer. It appears on screen at the
next frame of [the loop](the-loop.md).

## Colors

Colors are plain integers in the form **`0xRRGGBB`**: one byte each for red,
green and blue, from `00` to `FF`.

```python
RED    = 0xFF0000
ORANGE = 0xFF8000
GRAY   = 0x808080
```

Writing a helper to build colors from components is common:

```python
def rgb(r: int, g: int, b: int) -> int:
    """0-255 components -> 0xRRGGBB."""
    return (r << 16) | (g << 8) | b

rgb(255, 128, 0) == 0xFF8000  # True
```

And going the other way:

```python
r, g, b = (color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF
```

`pixel_put` and `string_put` ignore anything above the lowest 24 bits, so
`0xFF000000 | RED` draws plain red. The top byte only matters in
[images](#transparency).

## Single pixels

```python
malibx.pixel_put(mlx, win, x, y, color)
```

Sets one pixel. `(0, 0)` is the top-left corner. Coordinates outside the window
are ignored, so you can draw shapes that go off the edge without clipping
them yourself.

A few shapes built from `pixel_put`:

=== "Horizontal line"

    ```python
    def hline(x0, x1, y, color):
        for x in range(x0, x1 + 1):
            malibx.pixel_put(mlx, win, x, y, color)
    ```

=== "Filled rectangle"

    ```python
    def rect(x, y, w, h, color):
        for py in range(y, y + h):
            for px in range(x, x + w):
                malibx.pixel_put(mlx, win, px, py, color)
    ```

=== "Circle outline"

    ```python
    import math

    def circle(cx, cy, r, color):
        steps = max(8, int(2 * math.pi * r))
        for i in range(steps):
            a = 2 * math.pi * i / steps
            malibx.pixel_put(mlx, win,
                             round(cx + r * math.cos(a)),
                             round(cy + r * math.sin(a)), color)
    ```

=== "Gradient"

    ```python
    for x in range(640):
        shade = x * 255 // 639
        for y in range(480):
            malibx.pixel_put(mlx, win, x, y, rgb(shade, 0, 255 - shade))
    ```

    This is 307,200 calls, which is slow if you redraw it every frame. Draw it
    once into an [image](#images) instead.

### Clearing

```python
malibx.clear_window(mlx, win)
```

Fills the whole window with black. Windows keep whatever you drew until you
draw over it, so clear (or redraw a full-window background) at the start of
each frame when things move.

## Text

```python
malibx.string_put(mlx, win, x, y, color, "Score: 42")
```

Draws a string with malibx's built-in 8×8 bitmap font.

- **`(x, y)` is the left end of the baseline**, as in X11 and MiniLibX. With
  the default size, letters extend 7 pixels **above** `y` and 1 pixel below it.
  To put text at the very top of a window, use `y = 7` or more, not `y = 0`.
- Only the letter pixels are drawn. The background stays as it was, so text
  works on top of images.
- Each character is 8 pixels wide. A string of `n` characters is `8 * n`
  pixels wide.
- ASCII and Latin-1 characters (`é`, `ü`, `ñ`, `©`, …) are supported. Other
  characters are left blank, but still take up a character's width.

```text
          ┌────────┐
          │  ████  │ ┐
          │ ██  ██ │ │
          │ ██  ██ │ │ 7 px above the baseline
          │ ██████ │ │
          │ ██  ██ │ │
          │ ██  ██ │ ┘
 (x, y) ● ├────────┤ ← baseline
          │        │   1 px below
          └────────┘
             8 px
```

### Bigger text with `set_font`

```python
malibx.set_font(mlx, win, "16x16")
malibx.string_put(mlx, win, 20, 40, 0xFFFFFF, "BIG")
```

`set_font` sets the text size **for that window**, and it stays in effect
until you change it. malibx has one font, so the name chooses a size: `"8x8"`
(default), `"16x16"`, `"24x24"`, `"32x32"`, `"40x40"`, `"48x48"`, `"56x56"`,
or `"64x64"`. Each step scales the 8×8 font up in whole pixels, so it stays
sharp. Any other name raises `ValueError`.

Everything scales together. At `"16x16"`, characters are 16 px wide and rise
14 px above the baseline.

!!! example "Centering text"
    ```python
    def center_text(win, cx, cy, color, text, size=8):
        """Draw text centered on (cx, cy), at the window's current font size."""
        width = len(text) * size
        x = cx - width // 2
        y = cy + size * 7 // 8 - size // 2   # baseline so the glyphs are centered
        malibx.string_put(mlx, win, x, y, color, text)
    ```

## Images

An image is a block of pixels in memory that you fill however you like, then
**draw to a window in a single call**. Use images for anything big or
anything you draw every frame.

```python
img = malibx.new_image(mlx, width, height)              # black
buf, bpp, size_line, endian = malibx.get_data_addr(img)
# ... write pixels into buf ...
malibx.put_image_to_window(mlx, win, img, x, y)         # (x, y) = top-left
```

### The pixel buffer

`get_data_addr` returns four values, the same as MiniLibX:

| Value | What it is | In malibx |
|---|---|---|
| `buf` | A writable `memoryview` of the image's bytes | `len(buf) == size_line * height` |
| `bpp` | Bits per pixel | always `32` |
| `size_line` | Bytes per row | always `width * 4` |
| `endian` | Byte order | always `0` (little endian) |

Each pixel is **4 bytes in the order B, G, R, A**. Pixel `(x, y)` starts at:

```python
i = y * size_line + x * 4
```

```text
            pixel (0,0)       pixel (1,0)       pixel (2,0)
buf:  ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─ ─
      │ B  │ G  │ R  │ A  │ B  │ G  │ R  │ A  │ B  │ G  │ R  │ A  │ …
      └────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴─ ─
       i=0                 i=4                 i=8
      row y starts at i = y * size_line
```

That's the byte order of `0xAARRGGBB` stored little endian, so the easy way
to write a pixel is:

```python
buf[i:i + 4] = color.to_bytes(4, "little")
```

And the easy way to read one back:

```python
color = int.from_bytes(buf[i:i + 3], "little")   # 0xRRGGBB
```

### Filling an image fast

Python loops are slow, but slice assignment on a `memoryview` runs at C
speed. Build the bytes for a whole row (or the whole image) and assign them
at once:

=== "Whole image, one color"

    ```python
    buf[:] = (0x1A1030).to_bytes(4, "little") * (len(buf) // 4)
    ```

=== "A rectangle"

    ```python
    def fill_rect(buf, size_line, x, y, w, h, color):
        px = color.to_bytes(4, "little")
        for row in range(y, y + h):
            start = row * size_line + x * 4
            buf[start:start + w * 4] = px * w
    ```

=== "A gradient, row by row"

    ```python
    W, H = 640, 480
    img = malibx.new_image(mlx, W, H)
    buf, _, size_line, _ = malibx.get_data_addr(img)

    row = b"".join(rgb(x * 255 // (W - 1), 0, 255 - x * 255 // (W - 1))
                   .to_bytes(4, "little") for x in range(W))
    for y in range(H):
        buf[y * size_line:(y + 1) * size_line] = row
    ```

    The 307,200 `pixel_put` calls from before become 480 slice assignments.

!!! tip "Keep the image, redraw the image"
    Build images **once**, outside the loop, and only call
    `put_image_to_window` each frame. Drawing a full-window image each frame is
    cheap. Rebuilding it pixel by pixel each frame isn't.

### Transparency

When an image is drawn, a pixel whose **alpha byte is `0xFF` isn't drawn**,
so the window shows through. Any other alpha is drawn fully opaque. There's
no blending. This matches MiniLibX.

```python
CLEAR = (0xFF000000).to_bytes(4, "little")   # alpha 0xFF = transparent

sprite = malibx.new_image(mlx, 32, 32)
buf, _, size_line, _ = malibx.get_data_addr(sprite)
buf[:] = CLEAR * (32 * 32)                    # start fully transparent
fill_rect(buf, size_line, 8, 8, 16, 16, 0xFFD700)  # opaque yellow square
```

!!! warning "Opaque by default"
    `color.to_bytes(4, "little")` for a normal `0xRRGGBB` color leaves alpha
    at `0x00`, which means **opaque**. That's usually what you want. It's the
    reverse of most graphics libraries, where 0 alpha means invisible.

### Images from XPM files

[XPM](https://en.wikipedia.org/wiki/X_PixMap) is the text-based image format
MiniLibX uses. malibx can load it:

```python
img, width, height = malibx.xpm_file_to_image(mlx, "assets/skull.xpm")
malibx.put_image_to_window(mlx, win, img, 100, 50)
```

- `filename` can be a `str` or a `pathlib.Path`.
- XPM pixels with the color `None` become transparent.
- Colors can be `#RGB`, `#RRGGBB`, `#RRRRGGGGBBBB` or an X11 color name like
  `white` or `light blue`. Unknown names become black.
- A missing file raises `FileNotFoundError`, and a malformed one raises
  `ValueError`.

If the XPM data is already in your program, pass its strings as a list:

```python
SMILEY = [
    "8 8 3 1",      # width height colors chars-per-pixel
    ". c None",
    "y c #FFD700",
    "k c #000000",
    "..yyyy..",
    ".yyyyyy.",
    "yykyykyy",
    "yyyyyyyy",
    "ykyyyyky",
    "yykkkkyy",
    ".yyyyyy.",
    "..yyyy..",
]
img, w, h = malibx.xpm_to_image(mlx, SMILEY)
```

The image you get is an ordinary malibx image, so you can also edit it
through `get_data_addr`.

### Freeing images

```python
malibx.destroy_image(mlx, img)
```

This releases the image's memory straight away. Using the image afterwards
raises `RuntimeError: image is destroyed`. Python also frees images automatically when nothing refers
to them anymore, so you only need `destroy_image` to free a large image at a
specific moment, or for MiniLibX-style code.

## Putting it together

A starfield: a background image built once, a text overlay, and a
transparent sprite that moves with the frame counter.

```python title="scene.py" linenums="1"
import random
import malibx

W, H = 640, 480
mlx = malibx.init()
win = malibx.new_window(mlx, W, H, "scene")

# background: dark sky with random stars, built once
sky = malibx.new_image(mlx, W, H)
buf, _, line, _ = malibx.get_data_addr(sky)
buf[:] = (0x0A0618).to_bytes(4, "little") * (W * H)
for _ in range(300):
    x, y = random.randrange(W), random.randrange(H)
    buf[y * line + x * 4:y * line + x * 4 + 4] = (0xFFFFFF).to_bytes(4, "little")

# sprite: the XPM smiley from above
SMILEY = ["8 8 3 1", ". c None", "y c #FFD700", "k c #000000",
          "..yyyy..", ".yyyyyy.", "yykyykyy", "yyyyyyyy",
          "ykyyyyky", "yykkkkyy", ".yyyyyy.", "..yyyy.."]
smiley, sw, sh = malibx.xpm_to_image(mlx, SMILEY)

state = {"frame": 0}

def frame(st):
    f = st["frame"]
    malibx.put_image_to_window(mlx, win, sky, 0, 0)            # covers everything: no clear needed
    malibx.put_image_to_window(mlx, win, smiley, f % (W + sw) - sw, H // 2)
    malibx.string_put(mlx, win, 12, 20, 0xB040FF, f"frame {f}")
    st["frame"] += 1

malibx.loop_hook(mlx, frame, state)
malibx.loop(mlx)
```

---

Next: [Keyboard and mouse :material-arrow-right:](input.md)
