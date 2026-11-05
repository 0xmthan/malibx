# xpm_file_to_image

```python
malibx.xpm_file_to_image(mlx: Mlx, filename: str | os.PathLike[str]) -> tuple[Image, int, int]
```

<p class="mx-mlx">MiniLibX: <code>mlx_xpm_file_to_image</code></p>

Loads an `.xpm` image file.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `filename` | `str` or `PathLike` | Path to the `.xpm` file, relative to the current working directory |

## Returns

A tuple `(image, width, height)`. MiniLibX fills in `width` and `height`
through pointers; malibx returns them instead.

## Raises

| Exception | When |
|---|---|
| `FileNotFoundError` / `OSError` | The file doesn't exist or can't be read |
| `ValueError` | The XPM data is malformed, for example `xpm: pixel row shorter than 16 pixels` |

## Supported XPM

- XPM3 files: the usual C-array format, `static char *name[] = { "...", ... };`.
  Comments are skipped.
- Any number of characters per pixel.
- Colors from the `c` key, in these forms:

| Form | Example |
|---|---|
| `#RGB` | `#F80` |
| `#RRGGBB` | `#FF8800` |
| `#RRRRGGGGBBBB` | `#FFFF88880000` |
| X11 color name, case-insensitive | `white`, `light blue`, `DarkOrange` |
| `None` | transparent |

- `None` pixels get alpha `0xFF`, so they're skipped when the image is drawn.
  All other pixels are opaque.
- Unknown color names, and pixels that use a character missing from the color
  table, become **black**. This matches MiniLibX.

!!! tip "Paths relative to your script"
    Relative paths are resolved from the directory you *run* Python in, not
    the script's directory. To always find files next to your script:

    ```python
    from pathlib import Path
    HERE = Path(__file__).parent
    img, w, h = malibx.xpm_file_to_image(mlx, HERE / "assets" / "player.xpm")
    ```

## Example

```python
skull, w, h = malibx.xpm_file_to_image(mlx, "assets/skull.xpm")
malibx.put_image_to_window(mlx, win, skull, (640 - w) // 2, (480 - h) // 2)
```

## See also

[`xpm_to_image`](xpm_to_image.md) · [`put_image_to_window`](put_image_to_window.md)
