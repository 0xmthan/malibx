# key_hook

```python
malibx.key_hook(win: Window, f: Callable[[int, T], Any], param: T = None) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_key_hook</code></p>

Calls `f(keysym, param)` when a key is **released** in the window.

## Parameters

| Name | Type | Description |
|---|---|---|
| `win` | `Window` | The window to listen on |
| `f` | `Callable[[int, T], Any]` | Called as `f(keysym, param)` |
| `param` | `T` | Passed to `f`. Defaults to `None`. |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

!!! warning "Release, not press"
    This matches MiniLibX, where `mlx_key_hook` is KeyRelease. To react as
    soon as a key goes down, use [`hook`](hook.md) with event `2`:
    `malibx.hook(win, 2, 1 << 0, f, param)`.

- `keysym` is an X11 keysym. Letters are lowercase ASCII (`ord("a")`, even
  with ++shift++), digits are ASCII, and special keys are `0xFFxx`
  (++esc++ `0xFF1B`, arrows `0xFF51`–`0xFF54`). See the
  [key table](../getting-started/input.md#keys).
- It's the same as `hook(win, 3, 1 << 1, f, param)`, and the two replace
  each other.

## Example

```python
ESC = 0xFF1B

def on_key(keysym, _):
    print(f"released 0x{keysym:04x}")
    if keysym == ESC:
        malibx.loop_end(mlx)

malibx.key_hook(win, on_key)
```

## See also

[`hook`](hook.md) · [Keyboard and mouse](../getting-started/input.md)
