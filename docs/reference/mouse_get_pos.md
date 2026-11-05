# mouse_get_pos

```python
malibx.mouse_get_pos(mlx: Mlx, win: Window) -> tuple[int, int]
```

<p class="mx-mlx">MiniLibX: <code>mlx_mouse_get_pos</code></p>

Returns where the mouse pointer is, relative to a window.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `win` | `Window` | The window to measure from |

## Returns

A tuple `(x, y)` in window pixels, with `(0, 0)` at the top-left of the
drawing area. MiniLibX fills in `x` and `y` through pointers; malibx returns
them instead.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- When the pointer is **outside** the window, the values are still relative
  to it, so they can be negative or larger than the window. Check the bounds
  before using them as pixel coordinates.
- It's `(0, 0)` if the position isn't known yet, for example before the
  window's first frame.
- The position is refreshed every frame (and by [`do_sync`](do_sync.md)),
  not at the exact moment you call this.
- On Wayland, the position only updates while the pointer is over the window.

## Example

```python
def frame(st):
    mx, my = malibx.mouse_get_pos(mlx, win)
    malibx.clear_window(mlx, win)
    malibx.put_image_to_window(mlx, win, crosshair, mx - 8, my - 8)
```

## See also

[`mouse_move`](mouse_move.md) · [`hook`](hook.md) (event 6, MotionNotify)
