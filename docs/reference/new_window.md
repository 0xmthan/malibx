# new_window

```python
malibx.new_window(mlx: Mlx, size_x: int, size_y: int, title: str) -> Window
```

<p class="mx-mlx">MiniLibX: <code>mlx_new_window</code></p>

Opens a new window with a black drawing area.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `size_x` | `int` | Width of the drawing area, in pixels |
| `size_y` | `int` | Height of the drawing area, in pixels |
| `title` | `str` | Text shown in the title bar |

## Returns

A [`Window`](index.md#window) handle, to pass to the drawing and hook
functions.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window couldn't be created, for example because there's no display: `new_window failed: …` |
| `OverflowError` | `size_x` or `size_y` is negative |

## Notes

- The window starts **black**. Anything you draw before [`loop`](loop.md)
  runs is shown on its first frame.
- The window can't be resized. Its drawing area is always
  `size_x` × `size_y` pixels.
- Windows show the malibx skull as their icon: the Dock icon on macOS and the
  window icon on Linux X11. Wayland and Windows keep the system default.
- You can open as many windows as you like. Each has its own pixels, font size
  and hooks.
- There's no `destroy_window`. A window closes when the user clicks its close
  button. To close everything from code, call [`loop_end`](loop_end.md) and
  let the program exit.

## Example

```python
import malibx

mlx = malibx.init()
win = malibx.new_window(mlx, 800, 600, "My game")
malibx.loop(mlx)
```

## See also

[`init`](init.md) · [`clear_window`](clear_window.md) ·
[Your first window](../getting-started/first-window.md)
