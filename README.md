<h1 align="center"><code>providence</code></h1>

experimental csgo cheat

```rust
let global = Global::init()?;
let global2 = global.clone();

global.on_frame(move |frame| {
    if let Some(local_player) = global2.local_player() {
        local_player.view_angle().pitch = 89.0;
    }
});

global.on_move(move |mut movement| {
    movement.send_packet = movement.tick_count % 14 == 0;

    if !movement.local_player.flags().on_ground() {
        movement.in_jump = false;
    }

    if movement.in_duck {
        movement.in_fast_duck = true;
    }

    if movement.in_attack {
        movement.view_angle.yaw = -270.0;
        movement.view_angle.pitch = 89.0;
    }

    ...

    movement
});
```
