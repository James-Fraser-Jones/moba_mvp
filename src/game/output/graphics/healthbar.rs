use crate::game::*;
use bevy::prelude::*;
use std::sync::LazyLock;

pub struct HealthbarPlugin;
impl Plugin for HealthbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(
            Update,
            (add_healthbars, update_healthbars).in_set(UpdateGraphics),
        );
    }
}

const HEALTHBAR_ASPECT_RATIO: f32 = 5.;
const HEALTHBAR_WIDTH_SCALE: f32 = 2700.;
const HEALTHBAR_OFFSET: f32 = 5.;
const HEALTHBAR_INDICATOR_BORDER: f32 = 2.;
const HEALTHBAR_INDICATOR_HEALTH: f32 = 100.;
const HEALTHBAR_CULL_DISTANCE: f32 = 1000.;

static SUPPORTED_FONT_SIZES: LazyLock<Vec<f32>> = LazyLock::new(|| {
    let mut font_sizes = vec![11., 12., 13., 14., 18., 24., 30., 36., 48., 60., 72., 96.];
    font_sizes.reverse();
    font_sizes
});
fn get_largest_font_size(available_height: f32) -> Option<f32> {
    SUPPORTED_FONT_SIZES
        .iter()
        .skip_while(|x| **x > available_height)
        .next()
        .copied()
}

#[derive(Component, Copy, Clone, PartialEq, Eq, Default)]
pub enum DisplayHealthbar {
    #[default]
    Basic,
    Advanced,
}

#[derive(Component)]
struct HealthTextTag;

#[derive(Component)]
struct HealthbarAnchor(Entity);

fn init() {}

fn add_healthbars(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Health,
            &MaxHealth,
            Option<&Team>,
            &DisplayHealthbar,
        ),
        Added<DisplayHealthbar>,
    >,
) {
    for (entity, health, max_health, team, healthbar) in &mut query {
        let color = team_color(team.copied());
        let health_ratio = health.0 / max_health.0;
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
        if *healthbar == DisplayHealthbar::Advanced {
            healthbar_entity.with_children(|builder| {
                builder
                    //indicators
                    .spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            border: UiRect::all(Val::Px(HEALTHBAR_INDICATOR_BORDER))
                                .with_right(Val::Px(0.)),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        ..default()
                    })
                    .with_children(|builder| {
                        for _ in 0..(max_health.0 / HEALTHBAR_INDICATOR_HEALTH) as i32 {
                            builder.spawn(NodeBundle {
                                style: Style {
                                    flex_grow: 1.,
                                    height: Val::Percent(100.),
                                    border: UiRect::right(Val::Px(HEALTHBAR_INDICATOR_BORDER)),
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                ..default()
                            });
                        }
                        let remainder = max_health.0 % HEALTHBAR_INDICATOR_HEALTH;
                        if remainder > 0. {
                            builder.spawn(NodeBundle {
                                style: Style {
                                    flex_grow: remainder,
                                    height: Val::Percent(100.),
                                    border: UiRect::right(Val::Px(HEALTHBAR_INDICATOR_BORDER)),
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
                        builder.spawn((
                            TextBundle::from_section(
                                format!("{}", health.0),
                                TextStyle {
                                    font_size: 26.,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ),
                            HealthTextTag,
                        ));
                    });
            });
        }
    }
}

fn update_healthbars(
    mut healthbar_query: Query<
        (Entity, &mut Style, &HealthbarAnchor, &mut Visibility),
        Without<HealthTextTag>,
    >,
    display_query: Query<(&DisplayHealthbar, &DisplayModel, &Radius, &Transform)>,
    camera_query: Query<(&Camera, &Transform, &GlobalTransform), With<OrbitDistance>>,
    mut text_query: Query<(&mut Text, &mut Visibility), With<HealthTextTag>>,
    children_query: Query<&Children>,
) {
    let (camera, camera_transform, global_camera_transform) = camera_query.single();
    for (healthbar_entity, mut healthbar_style, healthbar_anchor, mut healthbar_visibility) in
        &mut healthbar_query
    {
        let (display_healthbar, display_model, display_radius, display_transform) =
            display_query.get(healthbar_anchor.0).unwrap();
        //choose precise anchor point based on anchor position and camera orientation
        let height = display_model.get_height(display_radius.0);
        let anchor_point =
            display_transform.translation + Vec3::ZERO.with_z(height + HEALTHBAR_OFFSET);
        //check healthbar anchor point is both within camera frustum and within cull range
        let pixel = position_to_pixel(anchor_point, camera, global_camera_transform);
        let distance_from_camera = (camera_transform.translation - anchor_point).length();
        if distance_from_camera >= HEALTHBAR_CULL_DISTANCE || pixel == None {
            //hide healthbar and text
            if *healthbar_visibility == Visibility::Visible {
                *healthbar_visibility = Visibility::Hidden;
                if *display_healthbar == DisplayHealthbar::Advanced {
                    let child = children_query
                        .iter_descendants(healthbar_entity)
                        .skip_while(|child| text_query.get(*child).is_err())
                        .next()
                        .unwrap();
                    let (_, mut visibility) = text_query.get_mut(child).unwrap();
                    *visibility = Visibility::Hidden;
                }
            }
        } else {
            *healthbar_visibility = Visibility::Visible;
            let pixel = pixel.unwrap();
            //set healthbar size and position
            let intended_width = HEALTHBAR_WIDTH_SCALE * display_radius.0 / distance_from_camera;
            let size = Vec2::new(intended_width, intended_width / HEALTHBAR_ASPECT_RATIO);
            healthbar_style.width = Val::Px(size.x);
            healthbar_style.height = Val::Px(size.y);
            healthbar_style.left = Val::Px(pixel.x - size.x / 2.);
            healthbar_style.top = Val::Px(pixel.y - size.y);
            //set text size
            if *display_healthbar == DisplayHealthbar::Advanced {
                let child = children_query
                    .iter_descendants(healthbar_entity)
                    .skip_while(|child| text_query.get(*child).is_err())
                    .next()
                    .unwrap();
                let (mut text, mut visibility) = text_query.get_mut(child).unwrap();
                if let Some(font_size) = get_largest_font_size(size.y) {
                    text.sections[0].style.font_size = font_size;
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
