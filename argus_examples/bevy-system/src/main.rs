use bevy::prelude::{App, Res, Resource};

fn main() {
    App::new().add_startup_system(startup_system).run();
}

#[derive(Resource)]
struct A;

fn startup_system(_: A) {}
