# string_put

```python
malibx.string_put(mlx: Mlx, win: Window, x: int, y: int, color: int, string: str) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_string_put</code></p>

Draws text in a window with the built-in bitmap font.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `win` | `Window` | The window to draw in |
| `x` | `int` | Left edge of the first character |
| `y` | `int` | The text's **baseline**, not its top |
| `color` | `int` | `0xRRGGBB` |
| `string` | `str` | The text to draw |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- **`y` is the baseline**, as in X11. With the default 8×8 font, glyphs
  extend 7 px above `y` and 1 px below it. Text drawn at `y = 0` is almost
  entirely off screen.
- Every character takes the same width: 8 px, multiplied by the
  [`set_font`](set_font.md) size factor. A string is `len(string) * 8` px wide
  at the default size.
- Only the glyph pixels are drawn, with no background box.
- ASCII and Latin-1 characters (`é`, `ñ`, `©`, …) are supported. Other
  characters, including `\n`, are drawn as blanks but still take up a
  character's width. To draw several lines, call `string_put` once per line.
- Text is clipped at the window edges.

## Example

```python
malibx.string_put(mlx, win, 10, 20, 0xFFFFFF, "Score: 42")

lines = ["line one", "line two"]
for i, line in enumerate(lines):
    malibx.string_put(mlx, win, 10, 40 + i * 12, 0xAAAAAA, line)
```

## See also

[`set_font`](set_font.md) · [Drawing: text](../getting-started/drawing.md#text)
