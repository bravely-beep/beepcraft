use avian3d::prelude::*;
use bevy::{pbr::CascadeShadowConfig, prelude::*};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = meshes.add(Cuboid::default());
    let capsule_mesh = meshes.add(Capsule3d::new(0.5, 0.7));
    let white_material = materials.add(StandardMaterial::from_color(Srgba::WHITE));
    let blue_material = materials.add(StandardMaterial::from_color(Srgba::BLUE));

    // Camera
    commands.spawn((
        Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
    ));
    // Light
    commands.spawn((
        Transform::from_xyz(-2.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfig::default(),
    ));
    // Floor
    commands.spawn((
        Transform::from_scale(Vec3::new(5.0, 1.0, 5.0)),
        RigidBody::Kinematic,
        Collider::cuboid(1.0, 1.0, 1.0),
        LinearVelocity::from(Vec3::Z),
        Mesh3d::from(cube_mesh.clone()),
        MeshMaterial3d::from(white_material),
    ));
    // Player
    commands.spawn((
        Transform::from_xyz(0.0, 3.0, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(0.5, 0.7),
        LockedAxes::ROTATION_LOCKED,
        Mesh3d::from(capsule_mesh),
        MeshMaterial3d::from(blue_material),
        Visibility::default(),
    ));
}


