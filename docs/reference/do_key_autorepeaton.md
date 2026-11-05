# do_key_autorepeaton

```python
malibx.do_key_autorepeaton(mlx: Mlx) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_do_key_autorepeaton</code></p>

Turns key auto-repeat back on, so KeyPress (event 2) fires again and again
while a key is held. This is the default.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |

## Returns

`None`.

## Notes

- Applies to every window of this `mlx`, from the next frame onwards.
- The repeat delay and speed come from the operating system's keyboard
  settings.
- Like [`do_key_autorepeatoff`](do_key_autorepeatoff.md), it only affects
  malibx, not the rest of the system.

## Example

```python
malibx.do_key_autorepeatoff(mlx)   # menu: one step per press
...
malibx.do_key_autorepeaton(mlx)    # text input: holding a key repeats it
```

## See also

[`do_key_autorepeatoff`](do_key_autorepeatoff.md) · [`hook`](hook.md)
