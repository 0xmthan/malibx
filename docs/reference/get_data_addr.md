# get_data_addr

```python
malibx.get_data_addr(img: Image) -> tuple[memoryview, int, int, int]
```

<p class="mx-mlx">MiniLibX: <code>mlx_get_data_addr</code></p>

Gives direct, writable access to an image's pixels.

## Parameters

| Name | Type | Description |
|---|---|---|
| `img` | `Image` | An image from [`new_image`](new_image.md) or an XPM loader |

## Returns

A tuple `(buffer, bits_per_pixel, size_line, endian)`:

| Value | Type | In malibx |
|---|---|---|
| `buffer` | `memoryview` | Writable view of the image's bytes, `size_line * height` long |
| `bits_per_pixel` | `int` | Always `32` |
| `size_line` | `int` | Bytes per row, always `width * 4` |
| `endian` | `int` | Always `0` (little endian) |

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The image was destroyed: `image is destroyed` |

## Pixel layout

Each pixel is 4 bytes, **B, G, R, A**. Pixel `(x, y)` starts at:

```python
i = y * size_line + x * (bits_per_pixel // 8)    # = y * size_line + x * 4
```

Converting colors:

```python
buf[i:i + 4] = color.to_bytes(4, "little")       # write 0xRRGGBB (alpha 0, opaque)
color = int.from_bytes(buf[i:i + 3], "little")   # read 0xRRGGBB
```

When the image is drawn, alpha `0xFF` means **transparent** and any other
alpha means opaque. See [`put_image_to_window`](put_image_to_window.md).

## Notes

- Writing to `buffer` changes the image itself. The changes appear in a
  window the next time you call `put_image_to_window`.
- Slice assignment (`buf[a:b] = row_bytes`) is far faster than writing pixels
  one at a time in a Python loop. Build a row's bytes once and copy them.
- `bits_per_pixel`, `size_line` and `endian` never change in malibx. They're
  returned so MiniLibX-style code that reads them works.

## Example

```python
img = malibx.new_image(mlx, 100, 50)
buf, bpp, size_line, endian = malibx.get_data_addr(img)

red = (0xFF0000).to_bytes(4, "little")
for y in range(50):
    buf[y * size_line:(y + 1) * size_line] = red * 100   # whole row at once
```

## See also

[`new_image`](new_image.md) · [`put_image_to_window`](put_image_to_window.md) ·
[Drawing: the pixel buffer](../getting-started/drawing.md#the-pixel-buffer)
