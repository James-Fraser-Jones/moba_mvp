use super::*;
use bevy::prelude::*;
use model::*;

const HEALTH_BAR_ASPECT_RATIO: f32 = 10.;
const HEALTH_BAR_WIDTH: f32 = 2700.;
const HEALTH_BAR_OFFSET: f32 = 5.;

#[derive(Component)]
pub struct DisplayHealth {
    anchor: Entity,
    basic: bool,
}
impl DisplayHealth {
    fn new(anchor: Entity, basic: bool) -> Self {
        Self { anchor, basic }
    }
}

pub fn add_healthbars(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Radius,
            &mut DisplayModel,
            Option<&Health>,
            Option<&Team>,
        ),
        Added<DisplayModel>,
    >,
) {
    for (entity, radius, display, health, team) in &mut query {
        if let Some(_) = health {
            // //advanced health bar
            // commands
            //     .spawn((
            //         NodeBundle {
            //             style: Style {
            //                 position_type: PositionType::Absolute,
            //                 padding: UiRect::all(Val::Px(2.)),
            //                 column_gap: Val::Px(2.),
            //                 ..default()
            //             },
            //             background_color: BackgroundColor(Color::BLACK),
            //             ..default()
            //         },
            //         DisplayUIAnchor(entity),
            //     ))
            //     .with_children(|builder| {
            //         builder.spawn(NodeBundle {
            //             style: Style {
            //                 height: Val::Percent(100.),
            //                 width: Val::Percent(100.),
            //                 ..default()
            //             },
            //             background_color: BackgroundColor(team_color(team.copied())),
            //             ..default()
            //         });
            //         builder.spawn(NodeBundle {
            //             style: Style {
            //                 height: Val::Percent(100.),
            //                 width: Val::Percent(100.),
            //                 ..default()
            //             },
            //             background_color: BackgroundColor(team_color(team.copied())),
            //             ..default()
            //         });
            //         builder.spawn(NodeBundle {
            //             style: Style {
            //                 height: Val::Percent(100.),
            //                 width: Val::Percent(100.),
            //                 ..default()
            //             },
            //             background_color: BackgroundColor(team_color(team.copied())),
            //             ..default()
            //         });
            //         builder.spawn(NodeBundle {
            //             style: Style {
            //                 height: Val::Percent(100.),
            //                 width: Val::Percent(72.),
            //                 ..default()
            //             },
            //             background_color: BackgroundColor(team_color(team.copied())),
            //             ..default()
            //         });
            //         builder.spawn(NodeBundle {
            //             style: Style {
            //                 height: Val::Percent(100.),
            //                 width: Val::Percent(300.), //need to ensure this value accounts for missing gap pixels to prevent distortion
            //                 ..default()
            //             },
            //             background_color: BackgroundColor(Color::BLACK),
            //             ..default()
            //         });
            //         builder.spawn(
            //             TextBundle::from_section(
            //                 "372",
            //                 TextStyle {
            //                     font_size: 36.,
            //                     color: Color::WHITE,
            //                     ..default()
            //                 },
            //             )
            //             .with_style(Style {
            //                 position_type: PositionType::Absolute,
            //                 height: Val::Percent(100.),
            //                 width: Val::Percent(100.),
            //                 ..default()
            //             })
            //             .with_text_justify(JustifyText::Center),
            //         );
            //     });

            //basic health bar
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK),
                        ..default()
                    },
                    DisplayHealth::new(entity, true),
                ))
                .with_children(|builder| {
                    builder.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(50.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        background_color: BackgroundColor(team_color(team.copied())),
                        ..default()
                    });
                });
        }
    }
}

pub fn anchor_healthbars(
    mut anchor_query: Query<(&mut Style, &DisplayHealth)>,
    display_query: Query<(&Transform, &DisplayModel, &Radius)>,
    camera_query: Query<(&Camera, &GlobalTransform, &Transform), With<OrbitDistance>>,
) {
    let (camera, global_transform, camera_transform) = camera_query.single();
    for (mut style, anchor) in &mut anchor_query {
        let (transform, display, radius) = display_query.get(anchor.anchor).unwrap();
        let mut elevation = display.height;
        if !display.raised {
            elevation /= 2.;
        }
        let ang = camera_transform.rotation.to_euler(EulerRot::ZYX).2;
        let anchor_point = transform.translation
            + Vec3::new(
                0.,
                radius.0 * ang.cos() + HEALTH_BAR_OFFSET,
                elevation * ang.sin() + HEALTH_BAR_OFFSET,
            );
        if let Some(pixel_point) = position_to_pixel(anchor_point, camera, global_transform) {
            let distance_from_camera = (camera_transform.translation - anchor_point).length();
            let intended_width = HEALTH_BAR_WIDTH * radius.0;
            let size = Vec2::new(intended_width, intended_width / HEALTH_BAR_ASPECT_RATIO)
                / distance_from_camera;
            style.width = Val::Px(size.x);
            style.height = Val::Px(size.y);
            style.left = Val::Px(pixel_point.x - size.x / 2.);
            style.top = Val::Px(pixel_point.y - size.y);
        }
    }
}
