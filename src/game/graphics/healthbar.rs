use super::*;
use bevy::prelude::*;
use model::*;

const HEALTH_BAR_ASPECT_RATIO: f32 = 8.;
const HEALTH_BAR_WIDTH: f32 = 2700.;
const HEALTH_BAR_OFFSET: f32 = 5.;
const HEALTH_INDICATOR_WIDTH: f32 = 4.;
const HEALTH_INDICATOR_AMOUNT: f32 = 100.;

#[derive(Component)]
pub struct DisplayHealthbar {
    basic: bool,
}
impl DisplayHealthbar {
    pub fn new(basic: bool) -> Self {
        Self { basic }
    }
}

#[derive(Component)]
pub struct HealthbarAnchor(Entity);

pub fn add_healthbars(
    mut commands: Commands,
    mut query: Query<(Entity, &Health, Option<&Team>, &DisplayHealthbar), Added<DisplayHealthbar>>,
) {
    for (entity, health, team, healthbar) in &mut query {
        let color = team_color(team.copied());
        let health_ratio = health.current / health.maximum;
        let mut healthbar_entity = commands
            //black bar
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                },
                HealthbarAnchor(entity),
            ));
        healthbar_entity.with_children(|builder| {
            //red bar
            builder.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(health_ratio * 100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: BackgroundColor(color),
                ..default()
            });
        });
        if !healthbar.basic {
            healthbar_entity.with_children(|builder| {
                builder
                    //indicators
                    .spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            border: UiRect::axes(
                                Val::Px(HEALTH_INDICATOR_WIDTH / 2.),
                                Val::Px(HEALTH_INDICATOR_WIDTH),
                            ),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        ..default()
                    })
                    .with_children(|builder| {
                        for _ in 0..(health.maximum / HEALTH_INDICATOR_AMOUNT) as i32 {
                            builder.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    border: UiRect::axes(
                                        Val::Px(HEALTH_INDICATOR_WIDTH / 2.),
                                        Val::Px(0.),
                                    ),
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                ..default()
                            });
                        }
                        let remainder = health.maximum % HEALTH_INDICATOR_AMOUNT;
                        if remainder > 0. {
                            builder.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(remainder),
                                    height: Val::Percent(100.),
                                    border: UiRect::axes(
                                        Val::Px(HEALTH_INDICATOR_WIDTH / 2.),
                                        Val::Px(0.),
                                    ),
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                ..default()
                            });
                        }
                    });
                builder
                    //text
                    .spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle::from_section(
                            format!("{}", health.current),
                            TextStyle {
                                font_size: 26.,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
            });
        }
    }
}

pub fn anchor_healthbars(
    mut anchor_query: Query<(&mut Style, &HealthbarAnchor)>,
    display_query: Query<(&Transform, &DisplayModel, &Radius)>,
    camera_query: Query<(&Camera, &GlobalTransform, &Transform), With<OrbitDistance>>,
) {
    let (camera, global_transform, camera_transform) = camera_query.single();
    for (mut style, anchor) in &mut anchor_query {
        let (transform, display, radius) = display_query.get(anchor.0).unwrap();
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
