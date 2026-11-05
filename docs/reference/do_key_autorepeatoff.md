# do_key_autorepeatoff

```python
malibx.do_key_autorepeatoff(mlx: Mlx) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_do_key_autorepeatoff</code></p>

Turns off key auto-repeat. KeyPress (event 2) then fires **once** per press
instead of repeating while the key is held.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |

## Returns

`None`.

## Notes

- Applies to every window of this `mlx`, from the next frame onwards.
- KeyRelease (event 3) and [`key_hook`](key_hook.md) aren't affected. They
  always fire once per release.
- **Unlike MiniLibX, it only affects malibx.** `mlx_do_key_autorepeatoff`
  calls `XAutoRepeatOff`, which turns off auto-repeat for the whole X
  session and stays off after the program exits. malibx just stops reporting
  repeats, so other apps and your terminal are untouched.
- Turn auto-repeat back on with
  [`do_key_autorepeaton`](do_key_autorepeaton.md).

## Example

```python
# A turn-based game: one step per key press, however long it's held.
malibx.do_key_autorepeatoff(mlx)

def key_down(key, st):
    if key == RIGHT:
        st["x"] += 1

malibx.hook(win, 2, 1 << 0, key_down, state)
```

## See also

[`do_key_autorepeaton`](do_key_autorepeaton.md) · [`hook`](hook.md) ·
[Smooth movement](../getting-started/input.md#smooth-movement)
