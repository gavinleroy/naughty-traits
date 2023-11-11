mod bevy_simplified;

use bevy_simplified::*;

struct Timer(usize);
impl Resource for Timer {}

fn run_timer(_: Timer) { /* ... */
}

fn main() {
    App::new()
        .insert_resource(Timer(0))
        .add_system(run_timer)
        .run();
}
