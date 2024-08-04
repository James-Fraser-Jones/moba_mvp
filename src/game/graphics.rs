use crate::game::consts::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

pub struct MaterialSettings {
    saturation: f32,
    unlit: bool,
    hues: [(&'static str, f32); 6],
    luminances: [(&'static str, f32); 2],
    alphas: [(&'static str, f32); 2],
}
impl Default for MaterialSettings {
    fn default() -> Self {
        Self {
            saturation: 0.75,
            unlit: true,
            hues: [
                ("red", 0.),
                ("green", 120.),
                ("blue", 240.),
                ("teal", 190.),
                ("yellow", 60.),
                ("purple", 275.),
            ],
            luminances: [("", 0.5), ("dark", 0.25)],
            alphas: [("", 1.), ("trans", 0.3)],
        }
    }
}

pub struct MapSettings {
    spawner_radius: f32,
    river_width: f32,
    unit_angle: f32,
}
impl Default for MapSettings {
    fn default() -> Self {
        Self {
            spawner_radius: 27.8,
            river_width: 200.,
            unit_angle: PI / 8.,
        }
    }
}

#[derive(Resource)]
struct MeshHandles(HashMap<String, Handle<Mesh>>);

#[derive(Resource)]
struct MaterialHandles(HashMap<String, Handle<StandardMaterial>>);

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //these could be consts somehow?
    let material_settings = MaterialSettings::default();
    let map_settings = MapSettings::default();

    commands.insert_resource(MeshHandles(HashMap::from([
        (
            "plain".to_string(),
            meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.))),
        ),
        (
            "river".to_string(),
            meshes.add(Cuboid::new(
                map_settings.river_width,
                f32::sqrt(2.) * NON_LANE_RADIUS * 2.,
                5.0,
            )),
        ),
        (
            "mid".to_string(),
            meshes.add(Cuboid::new(
                LANE_WIDTH,
                f32::sqrt(2.) * NON_LANE_RADIUS * 2.,
                10.0,
            )),
        ),
        (
            "lane".to_string(),
            meshes.add(Cuboid::new(LANE_WIDTH, 2000., 10.0)),
        ),
        (
            "base".to_string(),
            meshes.add(Extrusion::new(
                CircularSector::from_radians(map_settings.base_radius, 2. * PI / 4.),
                12.,
            )),
        ),
        (
            "spawner".to_string(),
            meshes.add(Sphere::new(map_settings.spawner_radius)),
        ),
        ("unit".to_string(), meshes.add(Sphere::new(UNIT_RADIUS))),
        (
            "direction".to_string(),
            meshes.add(Cone {
                radius: 2.
                    * UNIT_RADIUS
                    * map_settings.unit_angle.cos()
                    * map_settings.unit_angle.sin(),
                height: 2.
                    * UNIT_RADIUS
                    * map_settings.unit_angle.cos()
                    * map_settings.unit_angle.cos(),
            }),
        ),
    ])));

    let mut material_handles = MaterialHandles(HashMap::new());
    for (hue_name, hue) in material_settings.hues {
        for (luminance_name, luminance) in material_settings.luminances {
            for (alpha_name, alpha) in material_settings.alphas {
                let material = StandardMaterial {
                    base_color: Color::hsla(hue, material_settings.saturation, luminance, alpha),
                    unlit: material_settings.unlit,
                    alpha_mode: if alpha < 1. {
                        AlphaMode::Blend
                    } else {
                        AlphaMode::Opaque
                    },
                    ..default()
                };
                let mut name = alpha_name.to_string();
                name.push_str(&luminance_name);
                name.push_str(&hue_name);
                material_handles.0.insert(name, materials.add(material));
            }
        }
    }
    commands.insert_resource(material_handles);

    // pub fn spawn(self, commands: &mut Commands) -> Entity {
    //     commands
    //         .spawn(self)
    //         .with_children(|builder| {
    //             builder.spawn(MeshBundle::new(
    //                 "plain",
    //                 "dark_green",
    //                 vec4_to_trans(MID.extend(0.).extend(0.)),
    //             ));
    //             builder.spawn(MeshBundle::new(
    //                 "river",
    //                 "teal",
    //                 vec4_to_trans(MID.extend(2.5).extend(PI / 4.)),
    //             ));
    //             builder.spawn(MeshBundle::new(
    //                 "mid",
    //                 "yellow",
    //                 vec4_to_trans(MID.extend(5.).extend(-PI / 4.)),
    //             ));
    //             builder.spawn(MeshBundle::new(
    //                 "lane",
    //                 "yellow",
    //                 vec4_to_trans(RED_TOP.extend(5.).extend(0.)),
    //             ));
    //             builder.spawn(MeshBundle::new(
    //                 "lane",
    //                 "yellow",
    //                 vec4_to_trans(BLUE_TOP.extend(5.).extend(PI / 2.)),
    //             ));
    //             builder.spawn(MeshBundle::new(
    //                 "lane",
    //                 "yellow",
    //                 vec4_to_trans(RED_BOT.extend(5.).extend(PI / 2.)),
    //             ));
    //             builder.spawn(MeshBundle::new(
    //                 "lane",
    //                 "yellow",
    //                 vec4_to_trans(BLUE_BOT.extend(5.).extend(0.)),
    //             ));
    //             builder.spawn(MeshBundle::new(
    //                 "base",
    //                 "dark_red",
    //                 vec4_to_trans(Vec4::new(-1000., -1000., 6., -PI / 4.)),
    //             ));
    //             builder.spawn(MeshBundle::new(
    //                 "base",
    //                 "dark_blue",
    //                 vec4_to_trans(Vec4::new(1000., 1000., 6., 3. * PI / 4.)),
    //             ));
    //         })
    //         .id()
    // }
}

fn update() {
    // pub fn spawn(self, commands: &mut Commands) -> Entity {
    //     commands
    //         .spawn(self)
    //         .with_children(|builder| {
    //             builder.spawn(MeshBundle::new(
    //                 "spawner",
    //                 "purple",
    //                 vec4_to_trans(Vec4::new(0., 0., SPAWNER_RADIUS, 0.)),
    //             ));
    //         })
    //         .id()
    // }

    // pub fn spawn(self, commands: &mut Commands) -> Entity {
    //     let team = self.team;
    //     let team_string = match team {
    //         Team::Red => "red",
    //         Team::Blue => "blue",
    //     };
    //     let sight_layer = match team {
    //         Team::Red => CollisionLayer::RedSight,
    //         Team::Blue => CollisionLayer::BlueSight,
    //     };
    //     let attack_layer = match team {
    //         Team::Red => CollisionLayer::RedAttack,
    //         Team::Blue => CollisionLayer::BlueAttack,
    //     };
    //     let opposite_layer = match team {
    //         Team::Red => CollisionLayer::BlueUnit,
    //         Team::Blue => CollisionLayer::RedUnit,
    //     };
    //     let mut unit = commands.spawn(self);
    //     let id = unit.id().index().to_string();
    //     unit.with_children(|builder| {
    //         builder.spawn((
    //             Collider::circle(UNIT_SIGHT_RADIUS),
    //             Sensor,
    //             CollisionLayers::new(sight_layer, opposite_layer),
    //             SightCollider,
    //         ));
    //         builder.spawn((
    //             Collider::circle(UNIT_ATTACK_RADIUS),
    //             Sensor,
    //             CollisionLayers::new(attack_layer, opposite_layer),
    //             AttackCollider,
    //         ));
    //         builder.spawn(MeshBundle::new(
    //             "unit",
    //             "trans_green",
    //             vec4_to_trans(Vec4::new(0., 0., UNIT_RADIUS, 0.)),
    //         ));
    //         builder.spawn((MeshBundle::new(
    //             "direction",
    //             team_string,
    //             vec4_to_trans(Vec4::new(
    //                 UNIT_RADIUS * (1. - UNIT_TRIANGLE_ANGLE.cos().powf(2.)),
    //                 0.,
    //                 UNIT_RADIUS,
    //                 -PI / 2.,
    //             )),
    //         ),));
    //         builder.spawn((
    //             Text2dBundle {
    //                 text: Text::from_section(
    //                     id,
    //                     TextStyle {
    //                         font_size: 50.,
    //                         color: Color::WHITE,
    //                         ..default()
    //                     },
    //                 ),
    //                 ..default()
    //             },
    //             RenderLayers::layer(1),
    //         ));
    //     });
    //     unit.id()
    // }

    // fn update_orientations(mut query: Query<(&Action, &mut Transform, &LinearVelocity), With<Unit>>) {
    //     for (action, mut trans, linear_velocity) in &mut query {
    //         if let Action::Move(_, _) = *action {
    //             trans.rotation = Quat::from_rotation_z(linear_velocity.0.to_angle());
    //         } else if let Action::Attack(_, AttackBehaviour::Pursue) = *action {
    //             trans.rotation = Quat::from_rotation_z(linear_velocity.0.to_angle());
    //         }
    //     }
    // }
}
