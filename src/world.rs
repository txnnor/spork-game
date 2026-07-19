use bevy::prelude::*;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.7, 0.3))),
    ));

    commands.spawn((
        PointLight {
            contact_shadows_enabled: true,
            intensity: 12_000_000.0,
            radius: 80.0,
            color: Color::srgb(0.7, 0.7, 0.7),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 7.0, 0.0),
    ));
}
