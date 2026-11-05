# xpm_to_image

```python
malibx.xpm_to_image(mlx: Mlx, xpm_data: list[str]) -> tuple[Image, int, int]
```

<p class="mx-mlx">MiniLibX: <code>mlx_xpm_to_image</code></p>

Creates an image from XPM data that's already in your program.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `xpm_data` | `list[str]` | The XPM's strings, in order: the header, then one string per color, then one per pixel row |

## Returns

A tuple `(image, width, height)`.

## Raises

| Exception | When |
|---|---|
| `ValueError` | The data is malformed: a bad header, missing lines, or rows that are too short |

## The data format

```python
[
    "W H NCOLORS CPP",     # width, height, number of colors, characters per pixel
    "<chars> c <color>",   # NCOLORS lines
    "<row>",               # H lines, each W * CPP characters
]
```

Colors work as in [`xpm_file_to_image`](xpm_file_to_image.md#supported-xpm):
`#RRGGBB`, X11 names, or `None` for transparent.

## Example

```python
HEART = [
    "7 6 2 1",
    ". c None",
    "r c #E02040",
    ".rr.rr.",
    "rrrrrrr",
    "rrrrrrr",
    ".rrrrr.",
    "..rrr..",
    "...r...",
]
heart, w, h = malibx.xpm_to_image(mlx, HEART)
malibx.put_image_to_window(mlx, win, heart, 10, 10)
```

## See also

[`xpm_file_to_image`](xpm_file_to_image.md) · [`get_data_addr`](get_data_addr.md)
