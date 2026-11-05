# destroy_image

```python
malibx.destroy_image(mlx: Mlx, img: Image) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_destroy_image</code></p>

Frees an image's pixels straight away.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `img` | `Image` | The image to free |

## Returns

`None`.

## Notes

- After this, [`get_data_addr`](get_data_addr.md) and
  [`put_image_to_window`](put_image_to_window.md) raise
  `RuntimeError: image is destroyed` for this image.
- Destroying an image twice is harmless.
- You usually don't need it. Python frees an image automatically once nothing
  refers to it. Use it to release a big image at a specific moment, or to
  keep MiniLibX-style code unchanged.
- Images already drawn in a window stay there. The window has its own copy
  of the pixels.

## Example

```python
splash, _, _ = malibx.xpm_file_to_image(mlx, "splash.xpm")
malibx.put_image_to_window(mlx, win, splash, 0, 0)
malibx.destroy_image(mlx, splash)     # the window keeps showing it
```

## See also

[`new_image`](new_image.md)
