# set_font

```python
malibx.set_font(mlx: Mlx, win: Window, name: str) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_set_font</code></p>

Sets the text size that [`string_put`](string_put.md) uses in a window.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `win` | `Window` | The window whose text size to set |
| `name` | `str` | `"8x8"`, `"16x16"`, `"24x24"`, `"32x32"`, `"40x40"`, `"48x48"`, `"56x56"` or `"64x64"` |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `ValueError` | `name` isn't one of the sizes above |
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- malibx has a single 8×8 bitmap font. `set_font` scales it up by a whole
  number, so `"16x16"` draws every font pixel as a 2×2 block, and text stays
  sharp.
- The size is stored **per window** and stays until you change it. New windows
  start at `"8x8"`.
- It only affects text drawn afterwards. Text already in the window doesn't
  change.
- The baseline offset scales too. At `"NxN"`, glyphs rise `N * 7 // 8` px
  above `y`.
- MiniLibX takes an X11 font name here. malibx only accepts the size names
  above.

## Example

```python
malibx.set_font(mlx, win, "32x32")
malibx.string_put(mlx, win, 20, 60, 0xFFFFFF, "GAME OVER")
malibx.set_font(mlx, win, "8x8")
malibx.string_put(mlx, win, 20, 90, 0xAAAAAA, "press Enter to restart")
```

## See also

[`string_put`](string_put.md)
