# Your first window

This page builds a complete malibx program one step at a time. It covers the
three things every malibx program does: **connect**, **open a window**, and
**run the loop**.

## The whole program

```python title="first_window.py" linenums="1"
import malibx

mlx = malibx.init()                                    # (1)!
win = malibx.new_window(mlx, 640, 480, "my window")    # (2)!

malibx.pixel_put(mlx, win, 320, 240, 0xFFFFFF)         # (3)!
malibx.string_put(mlx, win, 20, 30, 0xB040FF, "hi!")   # (4)!

malibx.loop(mlx)                                       # (5)!
```

1. Creates the **connection handle**. Nearly every other function takes it as
   its first argument.
2. Opens a 640 × 480 window titled *my window* and returns a **window handle**.
3. Draws one white pixel in the middle of the window.
4. Draws purple text. The position `(20, 30)` is the left end of the text's
   *baseline*.
5. Shows the window and handles events until it's closed.

Run it with `python first_window.py`. A black window opens with a white dot in
the middle and "hi!" in the top-left corner.

## Step by step

### 1. Connect with `init`

```python
mlx = malibx.init()
```

`init()` returns an `Mlx` object, the equivalent of MiniLibX's `void *mlx_ptr`.
It keeps track of your windows and runs the main loop. You need only one per
program.

### 2. Open a window with `new_window`

```python
win = malibx.new_window(mlx, 640, 480, "my window")
```

| Parameter | Meaning |
|---|---|
| `mlx` | The connection from `init()` |
| `size_x`, `size_y` | Width and height of the drawing area, in pixels |
| `title` | Text in the title bar |

It returns a `Window`. A new window is **black**. You can open several windows,
and each one has its own pixels and its own event hooks.

!!! info "Not on screen yet"
    `new_window` creates the window, but nothing you draw is shown until the
    loop runs. That's why you can draw before calling `loop`.

### 3. Draw something

The window has a pixel grid with its origin in the **top-left corner**.
`x` grows to the right and `y` grows **downwards**:

```text
(0,0) ──────────────► x
  │
  │      (320,240) ●   ← middle of a 640×480 window
  │
  ▼
  y                       (639,479) is the bottom-right pixel
```

Colors are integers written as `0xRRGGBB`:

| Value | Color |
|---|---|
| `0xFF0000` | <span class="mx-swatch" style="--c:#ff0000"></span> red |
| `0x00FF00` | <span class="mx-swatch" style="--c:#00ff00"></span> green |
| `0x0000FF` | <span class="mx-swatch" style="--c:#0000ff"></span> blue |
| `0xFFFFFF` | <span class="mx-swatch" style="--c:#ffffff"></span> white |
| `0xB040FF` | <span class="mx-swatch" style="--c:#b040ff"></span> purple |

Drawing outside the window does nothing. It doesn't raise an error, so
you don't need to check bounds yourself.

[Drawing](drawing.md) covers pixels, text and images in detail.

### 4. Run the loop

```python
malibx.loop(mlx)
```

`loop` is where your program spends its time. It shows your windows,
redraws them about 60 times a second, and calls your event hooks. It
**returns** when:

- every window has been closed, or
- you call `malibx.loop_end(mlx)`, for example from a key handler.

Code after `loop(mlx)` runs once the loop ends, so you can use it to save
state or print a goodbye.

!!! tip "Stopping a stuck program"
    Press ++ctrl+c++ in the terminal. `loop` checks for it every frame and
    raises `KeyboardInterrupt`.

## Make it close on Esc

Most programs also close when you press ++esc++. Register a key hook that
ends the loop:

```python title="first_window.py" linenums="1" hl_lines="9-12 14"
import malibx

ESC = 0xFF1B  # X11 keysym for the Escape key

mlx = malibx.init()
win = malibx.new_window(mlx, 640, 480, "my window")
malibx.string_put(mlx, win, 20, 30, 0xB040FF, "press Esc to quit")

def on_key(keysym, param):
    if keysym == ESC:
        malibx.loop_end(mlx)

malibx.key_hook(win, on_key)

malibx.loop(mlx)
print("bye!")
```

The hook gets the key's code (an X11 *keysym*, as in MiniLibX) and the `param`
you registered it with. Here `param` is `None` because none was passed.
[Keyboard and mouse](input.md) explains keysyms and the other hooks.

## Two windows

Each window has its own drawing area:

```python
import malibx

mlx = malibx.init()
left = malibx.new_window(mlx, 300, 200, "left")
right = malibx.new_window(mlx, 300, 200, "right")

malibx.string_put(mlx, left, 20, 30, 0xFF4040, "I'm on the left")
malibx.string_put(mlx, right, 20, 30, 0x40A0FF, "I'm on the right")

malibx.loop(mlx)  # returns when both windows are closed
```

Drawing to a window after the user has closed it raises
`RuntimeError: window is closed`.

## Recap

- [x] `init()` once, to get the connection
- [x] `new_window(mlx, w, h, title)` for each window
- [x] Draw with `pixel_put` and `string_put`; `(0, 0)` is the top-left corner
- [x] `loop(mlx)` shows everything and runs until the windows close or `loop_end` is called

---

Next: [The main loop :material-arrow-right:](the-loop.md). This is how
anything on screen moves.
