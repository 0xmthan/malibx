# put_image_to_window

```python
malibx.put_image_to_window(mlx: Mlx, win: Window, img: Image, x: int, y: int) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_put_image_to_window</code></p>

Draws an image into a window, with its top-left corner at `(x, y)`.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `win` | `Window` | The window to draw in |
| `img` | `Image` | The image to draw |
| `x` | `int` | Window column for the image's left edge |
| `y` | `int` | Window row for the image's top edge |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed (`window is closed`) or the image was destroyed (`image is destroyed`) |

## Notes

- **Transparency:** pixels whose alpha byte is `0xFF` are skipped, and the
  window shows through. Every other pixel is copied as fully opaque. There's
  no blending. XPM `None` pixels have alpha `0xFF`.
- The image is clipped to the window, so `x` and `y` can be negative or past
  the edges. That's useful for sprites sliding in and out of view.
- The pixels are **copied** when you call it. Changing the image afterwards
  doesn't affect the window until you draw it again.
- It's fast: one call copies the whole image. Drawing a full-window
  background every frame is fine.

## Example

```python
def frame(st):
    malibx.put_image_to_window(mlx, win, background, 0, 0)   # covers everything
    malibx.put_image_to_window(mlx, win, player, st["x"], st["y"])
```

## See also

[`new_image`](new_image.md) · [`xpm_file_to_image`](xpm_file_to_image.md) ·
[Drawing: transparency](../getting-started/drawing.md#transparency)
