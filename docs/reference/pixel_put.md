# pixel_put

```python
malibx.pixel_put(mlx: Mlx, win: Window, x: int, y: int, color: int) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_pixel_put</code></p>

Sets one pixel of a window to a color.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `win` | `Window` | The window to draw in |
| `x` | `int` | Column, from `0` (left edge) to width − 1 |
| `y` | `int` | Row, from `0` (top edge) to height − 1 |
| `color` | `int` | `0xRRGGBB`, for example `0xFF0000` for red |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- `(0, 0)` is the **top-left** corner, and `y` grows downwards.
- Pixels outside the window are silently ignored, so shapes can go off the
  edge.
- Only the lowest 24 bits of `color` are used. Alpha doesn't apply to
  `pixel_put`.
- The pixel shows up on screen at the next frame of [`loop`](loop.md).
- Each call goes through Python, so drawing whole screens this way is slow
  (about 300,000 calls for 640×480). For large areas, fill an
  [image](new_image.md) and draw it with
  [`put_image_to_window`](put_image_to_window.md).

## Example

```python
# a white diagonal line
for i in range(200):
    malibx.pixel_put(mlx, win, i, i, 0xFFFFFF)
```

## See also

[`put_image_to_window`](put_image_to_window.md) ·
[Drawing: colors](../getting-started/drawing.md#colors)
