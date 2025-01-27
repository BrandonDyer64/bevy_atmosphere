use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_spectator::*;

fn main() {
    println!("Demonstrates changing the atmosphere model\n\t- G: Gradient\n\t- N: Nishita");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AtmospherePlugin)
        .add_plugin(SpectatorPlugin)
        .add_startup_system(setup)
        .add_system(change_model)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        AtmosphereCamera::default(),
        Spectator,
    ));
}

fn change_model(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::G) {
        info!("Changed to Gradient atmosphere model");
        commands.insert_resource(AtmosphereModel::new(Gradient::default()));
    } else if keys.just_pressed(KeyCode::N) {
        info!("Changed to Nishita atmosphere model");
        commands.insert_resource(AtmosphereModel::new(Nishita::default()));
    } else if keys.just_pressed(KeyCode::Key0) {
        info!("Reset atmosphere model to default");
        commands.remove_resource::<AtmosphereModel>();
    }
}
