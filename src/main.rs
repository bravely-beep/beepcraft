use avian3d::prelude::*;
use bevy::{ecs::system::SystemParam, pbr::CascadeShadowConfig, prelude::*};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_collision_hooks::<MyHooks>(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, prepare_walk)
        .add_systems(Update, |q: Query<&LinearVelocity, With<Player>>| q.iter().for_each(|v| println!("{:?}", v)))
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
    let red_material = materials.add(StandardMaterial::from_color(Srgba::RED));
    let blue_material = materials.add(StandardMaterial::from_color(Srgba::BLUE));

    // Camera
    commands.spawn((
        Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        PlayerCamera,
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
    // Physics cube
    // commands.spawn((
    //     Transform::from_xyz(0.0, 1.0, 0.0),
    //     RigidBody::Dynamic,
    //     Collider::cuboid(1.0, 1.0, 1.0),
    //     Mesh3d::from(cube_mesh),
    //     MeshMaterial3d::from(red_material),
    // ));
    // Player
    commands.spawn((
        Transform::from_xyz(0.0, 3.0, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(0.5, 0.7),
        LockedAxes::ROTATION_LOCKED,
        Player::default(),
        Mesh3d::from(capsule_mesh),
        MeshMaterial3d::from(blue_material),
        Visibility::default(),
    ));
}

#[derive(Component, Default)]
pub struct PlayerCamera;

#[derive(Component, Default)]
// #[require(ActiveCollisionHooks::MODIFY_CONTACTS)]
pub struct Player {
    /// In local space. Y component should be zero.
    walk_velocity: Vec3,
    jump: bool,
}

fn prepare_walk(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    camera_query: Query<&GlobalTransform, With<PlayerCamera>>,
    player_query: Query<(&mut Player, Entity, &GlobalTransform)>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };
    let yaw = Quat::from_rotation_y(camera_transform.rotation().to_euler(EulerRot::XYZ).1);
    let forward = yaw * -Vec3::Z;
    let right = yaw * -Vec3::X;

    let mut velocity = Vec3::ZERO;
    if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        velocity -= forward;
    };
    if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        velocity += forward;
    };
    if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        velocity -= right;
    };
    if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        velocity += right;
    };
    velocity = velocity.normalize_or_zero();
    velocity *= 2.0;

    let jump = keyboard_input.just_pressed(KeyCode::Space);

    for (mut player, player_entity, player_transform) in player_query {
        player.walk_velocity = player_transform.rotation() * velocity;
        player.jump = jump;
        // if (player.walk_velocity != Vec3::ZERO) || player.jump {
        //     commands.queue(WakeUpBody(player_entity));
        // }
    }
}

#[derive(SystemParam)]
struct MyHooks<'w, 's> {
    player_query: Query<'w, 's, (&'static Player, &'static GlobalTransform)>,
}

impl<'w, 's> CollisionHooks for MyHooks<'w, 's> {
    fn modify_contacts(&self, contacts: &mut ContactPair, _commands: &mut Commands) -> bool {
        // for (collider, scale) in [(contacts.collider1, -1.0), (contacts.collider2, 1.0)] {
        //     if let Ok((player, player_transform)) = self.player_query.get(collider) {
        //         let velocity = player_transform.rotation().inverse() * player.walk_velocity * scale;
        //         for manifold in &mut contacts.manifolds {
        //             let normal = manifold.normal * scale;
        //             // Check if standing on flat surface
        //             if normal.dot(Vec3::Y) > 0.95 {
        //                 manifold.tangent_velocity = velocity;
        //                 // manifold.friction = 1.0;
        //                 if player.jump {
        //                     println!("jump");
        //                 }
        //             }
        //         }
        //     }
        // }
        true
    }
}
