use num::clamp;
use bevy::{
    prelude::*,
    input::{keyboard::KeyCode, Input},
    window::WindowMode
};
use bevy_inspector_egui::WorldInspectorPlugin;

pub const HEIGHT: f32 = 1080.0;
pub const WIDTH: f32 = 1920.0;
pub const YLIMIT: f32 = 0.763;
pub const ZLIMIT: f32 = 1.445;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            // width: WIDTH,
            // height: HEIGHT,
            mode: WindowMode::Fullscreen,
            title: "pong".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_paddle_pong)
        .add_system(bevy::window::close_on_esc)
        .add_system(get_player_input)
        .add_system(move_pong)
        .run()
}

fn spawn_camera(mut commands: Commands){
    // Spawns the camera bundle, which is a structure that holds the camera components
    commands.spawn_bundle(Camera3dBundle {
        // set the position of the camera
        transform: Transform::from_xyz(-2.0, 0.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
    });
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct IsPong {
    zspeed: f32,
    yspeed: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct PlayerControllable {
    speed: f32,
    name: String,
}

fn spawn_paddle_pong(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ){
    commands.spawn_bundle(PbrBundle {
        // Use the cube mesh that comes stardard with bevy
        mesh: meshes.add(Mesh::from(shape::Cube {size: 0.025})),
        // Generate and use a plain color material
        material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
        // Move the cube to be above the plane
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(IsPong{
        zspeed: 1.0,
        yspeed: 1.0,
    })
    .insert(Name::new("pong"));

    commands.spawn_bundle(PbrBundle {
        // Use the cube mesh that comes stardard with bevy
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: -0.0125,
            max_x: 0.0125,
            min_y: -0.06,
            max_y: 0.06,
            min_z: -0.0125,
            max_z: 0.0125,
        })),
        // Generate and use a plain color material
        material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
        // Move the cube to be above the plane
        transform: Transform::from_xyz(0.0, 0.0, 1.2),
        ..default()
    })
    .insert(PlayerControllable {
        speed: 1.0,
        name: String::from("right paddle")
    })
    .insert(Name::new("right paddle"));

    commands.spawn_bundle(PbrBundle {
        // Use the cube mesh that comes stardard with bevy
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: -0.0125,
            max_x: 0.0125,
            min_y: -0.06,
            max_y: 0.06,
            min_z: -0.0125,
            max_z: 0.0125,
        })),
        // Generate and use a plain color material
        material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
        // Move the cube to be above the plane
        transform: Transform::from_xyz(0.0, 0.0, -1.2),
        ..default()
    })
    .insert(PlayerControllable {
        speed: 1.0,
        name: String::from("left paddle")
    })
    .insert(Name::new("left paddle"));

    // Spawn the point light bundle, holds light components such as intensity and shadows
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight{
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 0.0, 0.0),
        ..default()
    })
    .insert(Name::new("Light"));
}

fn get_player_input(
    mut player_query: Query<(&PlayerControllable, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    ){
    for (player, mut transform) in &mut player_query {
        if player.name.eq("right paddle") {
            if keyboard.pressed(KeyCode::Up){
                transform.translation.y = clamp(transform.translation.y + (0.5 * player.speed * time.delta_seconds()), -YLIMIT, YLIMIT);
            }
            else if keyboard.pressed(KeyCode::Down){
                transform.translation.y = clamp(transform.translation.y - (0.5 * player.speed * time.delta_seconds()), -YLIMIT, YLIMIT);
            }
        }
        else if player.name.eq("left paddle") {
            if keyboard.pressed(KeyCode::W){
                transform.translation.y = clamp(transform.translation.y + (0.5 * player.speed * time.delta_seconds()), -YLIMIT, YLIMIT);
            }
            else if keyboard.pressed(KeyCode::S){
                transform.translation.y = clamp(transform.translation.y - (0.5 * player.speed * time.delta_seconds()), -YLIMIT, YLIMIT);
            }
        }
    }
}

fn move_pong (
        mut pong_query: Query<(&mut IsPong, &mut Transform)>,
        time: Res<Time>,
        ) {
    let (mut ispong, mut transform) = pong_query.single_mut();
    transform.translation.y += 0.5 * ispong.yspeed * time.delta_seconds();
    transform.translation.z += 0.5 * ispong.zspeed * time.delta_seconds();
    if transform.translation.z < -ZLIMIT || transform.translation.z > ZLIMIT {
        transform.translation.z = 0.0;
    }
    else if transform.translation.y < -YLIMIT || transform.translation.y > YLIMIT {
        ispong.yspeed = -ispong.yspeed;
    }
}
