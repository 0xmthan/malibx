# mouse_hide

```python
malibx.mouse_hide(mlx: Mlx, win: Window) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_mouse_hide</code></p>

Hides the mouse pointer while it's over the window.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `win` | `Window` | The window to hide the pointer in |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- The pointer still moves and still produces events.
  [`mouse_get_pos`](mouse_get_pos.md) and the mouse hooks keep working.
- Hiding twice is the same as hiding once. One
  [`mouse_show`](mouse_show.md) always brings it back.
- Useful for games that draw their own cursor, or that use mouse-look with
  [`mouse_move`](mouse_move.md).

## Example

```python
malibx.mouse_hide(mlx, win)

def frame(st):
    mx, my = malibx.mouse_get_pos(mlx, win)
    malibx.put_image_to_window(mlx, win, background, 0, 0)
    malibx.put_image_to_window(mlx, win, cursor_img, mx, my)   # custom cursor
```

## See also

[`mouse_show`](mouse_show.md) · [`mouse_move`](mouse_move.md)
