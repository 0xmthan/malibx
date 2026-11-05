import malibx

mlx = malibx.init()
win = malibx.new_window(mlx, 640, 480, "malibx")
malibx.loop_hook(mlx, lambda param: None)
malibx.loop(mlx)
