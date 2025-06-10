use avian3d::prelude::*;
use bevy::{ecs::system::SystemParam, pbr::CascadeShadowConfig, prelude::*};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_collision_hooks::<MyHooks>(),
        ))
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
    let red_material = materials.add(StandardMaterial::from_color(Srgba::RED));
    let blue_material = materials.add(StandardMaterial::from_color(Srgba::BLUE));

    // Camera
    commands.spawn((
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
        Mesh3d::from(cube_mesh.clone()),
        MeshMaterial3d::from(white_material),
    ));
    // Physics cube
    commands.spawn((
        Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 1.0, 1.2, 0.3))
            .with_scale(Vec3::splat(0.5)),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Mesh3d::from(cube_mesh),
        MeshMaterial3d::from(red_material),
    ));
    // Player
    commands.spawn((
        Transform::from_xyz(1.5, 4.0, 1.5).looking_at(Vec3::new(0.0, 4.0, 0.0), Vec3::Y),
        RigidBody::Dynamic,
        Collider::capsule(0.5, 0.7),
        LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        Player,
        Mesh3d::from(capsule_mesh),
        MeshMaterial3d::from(blue_material),
        Visibility::default(),
    ));
}

#[derive(Component)]
#[require(ActiveCollisionHooks::MODIFY_CONTACTS)]
pub struct Player;

#[derive(SystemParam)]
struct MyHooks<'w, 's> {
    player_query: Query<'w, 's, &'static Player>,
}

impl<'w, 's> CollisionHooks for MyHooks<'w, 's> {
    fn modify_contacts(&self, contacts: &mut ContactPair, _commands: &mut Commands) -> bool {
        let velocity_scale = match (
            self.player_query.get(contacts.collider1),
            self.player_query.get(contacts.collider2),
        ) {
            (Ok(_), Ok(_)) => panic!("Two players exist?!"),
            (Ok(_player), Err(_)) => 1.0,
            (Err(_), Ok(_player)) => -1.0,
            (Err(_), Err(_)) => return true,
        };
        for manifold in &mut contacts.manifolds {
            manifold.tangent_velocity = velocity_scale * Vec3::Z;
        }
        true
    }
}
