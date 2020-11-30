use bevy::prelude::*;

/* Bevy uses ECS --- Entity Component System */

// Newtype pattern
struct Name(String);

// Our first component --- unit struct
struct Person;

struct GreetTimer(Timer);

// Our first normal system
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&Person, &Name)>) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        for (_person, name) in query.iter() {
            println!("hello, {}!", name.0);
        }
    }
}

// Our first startup system
fn add_people(mut commands: Commands) {
    commands
        .spawn((Person {}, Name("Elaina Proctor".to_string())))
        .spawn((Person {}, Name("Renzo Hume".to_string())))
        .spawn((Person {}, Name("Zayna Nieves".to_string())));
}

// Our first plugin
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
        // Add plugins
        .add_plugins(DefaultPlugins)
        // which is equivalent to
        // .add_plugin(CorePlugin::default())
        // .add_plugin(InputPlugin::default())
        // .add_plugin(WindowPlugin::default())
        // ... and others
        .add_plugin(HelloPlugin)
        // Add systems
        .run()

    // ? How is it possible to call a function on a function name as in `hello_world.system()`?
}
