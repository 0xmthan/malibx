# clear_window

```python
malibx.clear_window(mlx: Mlx, win: Window) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_clear_window</code></p>

Fills the whole window with black.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `win` | `Window` | The window to clear |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- Windows keep their pixels between frames. If things move, clear at the
  start of each frame in your [loop hook](loop_hook.md), then redraw.
- If you draw a background image that covers the whole window every frame,
  you don't need to clear.
- Black is the only option. For another background color, put a
  full-window [image](new_image.md) filled with that color.

## Example

```python
def frame(st):
    malibx.clear_window(mlx, win)
    malibx.put_image_to_window(mlx, win, player, st["x"], st["y"])
```

## See also

[`pixel_put`](pixel_put.md) · [`put_image_to_window`](put_image_to_window.md)
