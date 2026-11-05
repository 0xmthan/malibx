# new_image

```python
malibx.new_image(mlx: Mlx, width: int, height: int) -> Image
```

<p class="mx-mlx">MiniLibX: <code>mlx_new_image</code></p>

Creates an off-screen image: a block of pixels you can fill quickly and draw
to windows in one call.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `width` | `int` | Width in pixels, at least 1 |
| `height` | `int` | Height in pixels, at least 1 |

## Returns

An [`Image`](index.md#image). Every pixel starts as opaque black (all bytes
`0`).

## Raises

| Exception | When |
|---|---|
| `ValueError` | `width` or `height` is 0: `image size must be at least 1x1` |
| `OverflowError` | `width` or `height` is negative |

## Notes

- An image isn't tied to a window. You can draw it to any window, as many
  times as you like.
- Get its pixels with [`get_data_addr`](get_data_addr.md), and draw it with
  [`put_image_to_window`](put_image_to_window.md).
- Python frees the image when nothing refers to it any more.
  [`destroy_image`](destroy_image.md) frees it straight away.

## Example

```python
W, H = 640, 480
bg = malibx.new_image(mlx, W, H)
buf, _, _, _ = malibx.get_data_addr(bg)
buf[:] = (0x202830).to_bytes(4, "little") * (W * H)   # fill with dark blue-gray
malibx.put_image_to_window(mlx, win, bg, 0, 0)
```

## See also

[`get_data_addr`](get_data_addr.md) · [`put_image_to_window`](put_image_to_window.md) ·
[Drawing: images](../getting-started/drawing.md#images)
