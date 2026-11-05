# get_color_value

```python
malibx.get_color_value(mlx: Mlx, color: int) -> int
```

<p class="mx-mlx">MiniLibX: <code>mlx_get_color_value</code></p>

Converts a color to the format the display uses.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `color` | `int` | A color, `0xRRGGBB` |

## Returns

The color as `0xRRGGBB`, with anything above the lowest 24 bits removed.

## Notes

- In MiniLibX, displays can store pixels in different formats, so you convert
  colors before writing them into image memory. malibx always uses
  `0xRRGGBB`, so the color comes back unchanged, apart from the dropped top
  byte.
- It exists so MiniLibX code that calls it keeps working. New code doesn't
  need it.
- Dropping the top byte also drops alpha. To write a transparent pixel into
  an image, set the alpha byte yourself (see
  [`get_data_addr`](get_data_addr.md#pixel-layout)).

## Example

```python
malibx.get_color_value(mlx, 0xFF8000)     # 0xFF8000
malibx.get_color_value(mlx, 0xAAFF8000)   # 0xFF8000
```

## See also

[`get_data_addr`](get_data_addr.md) · [Drawing: colors](../getting-started/drawing.md#colors)
