use bevy::prelude::*;

/* Bevy uses ECS --- Entity Component System */

// newtype pattern
struct Name(String);

// Our first component --- unit struct
struct Person;

// Our first startup system
fn add_people(mut commands: Commands) {
    commands
        .spawn((Person {}, Name("Elaina Proctor".to_string())))
        .spawn((Person {}, Name("Renzo Hume".to_string())))
        .spawn((Person {}, Name("Zayna Nieves".to_string())));
}

// Our first system
fn hello_world() {
    println!("Hello, world!");
}

// Our first normal system
fn greet_people(_person: &Person, name: &Name) {
    println!("hello, {}!", name.0);
}

fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run()

    // ? How is it possible to call a function on a function name as in `hello_world.system()`?
}
