use bevy::prelude::*;

/* Bevy uses ECS */

// Our first system
fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    App::build()
        .add_system(hello_world.system())
        .run();

    // ? How is it possible to call a function on a function name as in `hello_world.system()`?
}
