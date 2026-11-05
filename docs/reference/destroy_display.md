# destroy_display

```python
malibx.destroy_display(mlx: Mlx) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_destroy_display</code></p>

Closes every window of this connection and removes its loop hook.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection to shut down |

## Returns

`None`.

## Notes

- Every open window closes immediately, as if you'd called
  [`destroy_window`](destroy_window.md) on each one. No DestroyNotify hooks
  are called.
- The [loop hook](loop_hook.md) is removed.
- Called while [`loop`](loop.md) is running, the loop returns at the end of
  the current frame, since no windows are left.
- In MiniLibX the connection can't be used afterwards, and you `free` it
  yourself. In malibx it stays usable: you can open new windows on it. Python
  frees it when it's no longer referenced.
- You don't need it for a clean exit. Python closes everything when the
  program ends.

## Example

```python
def quit_game(st):
    save(st)
    malibx.destroy_display(mlx)   # all windows close, loop returns
```

## See also

[`destroy_window`](destroy_window.md) · [`loop_end`](loop_end.md)
