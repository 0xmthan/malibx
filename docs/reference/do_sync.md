# do_sync

```python
malibx.do_sync(mlx: Mlx) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_do_sync</code></p>

Shows everything drawn so far on screen right away, without waiting for the
next frame of [`loop`](loop.md).

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window system failed to show a window (rare) |

## Notes

- Every open window is shown, just as one frame of `loop` would show it.
  Like a frame, it can wait a moment to stay within 60 fps.
- Input that arrives during `do_sync` isn't lost. It's kept, and the event
  hooks get it at the loop's next event step. No hooks run inside `do_sync`.
- Windows the user closed are removed, and their DestroyNotify (event 17) is
  delivered by the loop, as usual.
- Inside `loop` you rarely need it, because every frame is shown anyway. It's
  useful *before* `loop`, for example to show a loading screen during slow
  setup.

## Example

```python
malibx.string_put(mlx, win, 20, 30, 0xFFFFFF, "loading…")
malibx.do_sync(mlx)            # visible now, before the slow part

world = build_world()          # takes a few seconds
malibx.loop_hook(mlx, frame, world)
malibx.loop(mlx)
```

## See also

[`loop`](loop.md) · [The main loop](../getting-started/the-loop.md)
