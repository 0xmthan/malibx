# mouse_move

```python
malibx.mouse_move(mlx: Mlx, win: Window, x: int, y: int) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_mouse_move</code></p>

Moves the mouse pointer to `(x, y)` in a window.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `win` | `Window` | The window that `x` and `y` are relative to |
| `x` | `int` | Column in the window, from its left edge |
| `y` | `int` | Row in the window, from its top edge |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- Coordinates are in the window's drawing area, the same as everywhere else
  in malibx. Values outside the window move the pointer outside it.
- It moves the real system pointer.
- The new position shows up in [`mouse_get_pos`](mouse_get_pos.md) and in
  MotionNotify (event 6) from the next frame.
- **Wayland** doesn't let applications move the pointer, so there it does
  nothing. It works on macOS, Windows and Linux X11.

!!! example "Mouse-look"
    First-person games like *cub3D* turn the camera by how far the mouse
    moved, then put the pointer back in the center:

    ```python
    CX, CY = W // 2, H // 2

    def frame(st):
        mx, _ = malibx.mouse_get_pos(mlx, win)
        st["angle"] += (mx - CX) * 0.003
        malibx.mouse_move(mlx, win, CX, CY)
        draw(st)

    malibx.mouse_hide(mlx, win)
    malibx.mouse_move(mlx, win, CX, CY)
    ```

## See also

[`mouse_get_pos`](mouse_get_pos.md) · [`mouse_hide`](mouse_hide.md)
