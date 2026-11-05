# get_screen_size

```python
malibx.get_screen_size(mlx: Mlx) -> tuple[int, int]
```

<p class="mx-mlx">MiniLibX: <code>mlx_get_screen_size</code></p>

Returns the size of the main screen.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |

## Returns

A tuple `(width, height)`. MiniLibX fills in `sizex` and `sizey` through
pointers; malibx returns them instead.

| Platform | Measured in |
|---|---|
| macOS | Points, the same unit as window sizes. A Retina screen reports half its physical pixels. |
| Linux | Pixels of the X11 default screen. On Wayland this goes through XWayland. |
| Windows | Pixels of the primary monitor |

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | No display is available: `get_screen_size failed: no display` |

## Notes

- With several monitors, this is the **main** one: the one with the menu bar
  on macOS, the primary monitor on Windows, and the default screen on X11.
- It doesn't subtract the menu bar, Dock or taskbar. A window of exactly
  this size may be partly covered.

## Example

```python
sw, sh = malibx.get_screen_size(mlx)
w, h = min(1280, sw - 100), min(800, sh - 100)   # fit on small screens
win = malibx.new_window(mlx, w, h, "fits")
```

## See also

[`new_window`](new_window.md)
