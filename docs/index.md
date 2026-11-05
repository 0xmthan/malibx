---
hide:
  - navigation
  - toc
---

<div class="mx-hero" markdown>
![malibx](assets/logo.png)

# malibx

A tiny window and pixel library for Python, written in Rust, with a
[MiniLibX](https://github.com/42Paris/minilibx-linux)-style API.
Runs on macOS, Linux and Windows.

[Get started](getting-started/installation.md){ .md-button .md-button--primary }
[GitHub](https://github.com/0xmthan/malibx){ .md-button }
</div>

```python
import malibx

mlx = malibx.init()
win = malibx.new_window(mlx, 640, 480, "hello")
malibx.string_put(mlx, win, 20, 30, 0xB040FF, "hello, pixels")
malibx.loop(mlx)
```

That's a whole program: a window, some text, and an event loop that runs until
you close it.

<div class="grid cards" markdown>

-   :material-download:{ .lg .middle } **Install in one line**

    ---

    Prebuilt wheels for every major platform. No compiler and no X server setup.

    [:octicons-arrow-right-24: Installation](getting-started/installation.md)

-   :material-application-outline:{ .lg .middle } **Your first window**

    ---

    Open a window, draw on it, and close it cleanly, step by step.

    [:octicons-arrow-right-24: Your first window](getting-started/first-window.md)

-   :material-sync:{ .lg .middle } **The main loop**

    ---

    How frames, hooks and events fit together. Read this before you build
    anything that moves.

    [:octicons-arrow-right-24: The main loop](getting-started/the-loop.md)

-   :material-draw:{ .lg .middle } **Drawing**

    ---

    Pixels, colors, text, and fast off-screen images.

    [:octicons-arrow-right-24: Drawing](getting-started/drawing.md)

-   :material-keyboard-outline:{ .lg .middle } **Keyboard and mouse**

    ---

    Respond to keys, clicks and the close button.

    [:octicons-arrow-right-24: Keyboard and mouse](getting-started/input.md)

-   :material-school-outline:{ .lg .middle } **Coming from MiniLibX?**

    ---

    Same function names, without the `mlx_` prefix. `mlx_pixel_put(mlx, win, x, y, c)`
    becomes `malibx.pixel_put(mlx, win, x, y, c)`.

    [:octicons-arrow-right-24: Start here](getting-started/first-window.md)

</div>

## Why malibx?

- **Small.** The 30 functions of MiniLibX, no more. You can learn the whole API in an afternoon.
- **Direct.** You get a window and a buffer of pixels. There's no scene graph,
  sprite engine or framework to learn first.
- **Familiar.** It follows MiniLibX, the graphics library used by 42 schools,
  so code and ideas from C projects like *so_long*, *fract-ol*, *FdF* and
  *cub3D* carry straight over.
- **Portable.** The same script runs on macOS, Linux and Windows.
