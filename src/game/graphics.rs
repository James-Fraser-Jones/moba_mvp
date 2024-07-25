use crate::helpers::{consts::*, types::*, utils::*};
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::f32::consts::PI;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_assets);
        app.add_systems(Update, add_assets);
    }
}

//asset handles
#[derive(Resource)]
pub struct Handles {
    pub red: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
    pub yellow: Handle<ColorMaterial>,
    pub teal: Handle<ColorMaterial>,
    pub purple: Handle<ColorMaterial>,

    pub plain: Mesh2dHandle,
    pub river: Mesh2dHandle,
    pub mid: Mesh2dHandle,
    pub lane: Mesh2dHandle,
    pub base: Mesh2dHandle,

    pub spawner: Mesh2dHandle,

    pub unit: Mesh2dHandle,
    pub direction: Mesh2dHandle,
}

fn init_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Handles {
        //colors
        red: materials.add(Color::hsl(RED_HUE, SATURATION, BRIGHTNESS)),
        green: materials.add(Color::hsl(GREEN_HUE, SATURATION, BRIGHTNESS)),
        blue: materials.add(Color::hsl(BLUE_HUE, SATURATION, BRIGHTNESS)),
        yellow: materials.add(Color::hsl(YELLOW_HUE, SATURATION, BRIGHTNESS)),
        teal: materials.add(Color::hsl(TEAL_HUE, SATURATION, BRIGHTNESS)),
        purple: materials.add(Color::hsl(PURPLE_HUE, SATURATION, BRIGHTNESS)),

        //map
        plain: Mesh2dHandle(meshes.add(Rectangle::from_length(MAP_SIZE))),
        river: Mesh2dHandle(meshes.add(Rectangle::new(
            RIVER_WIDTH * MAP_SIZE,
            f32::sqrt(2.) * NON_LANE_WIDTH * MAP_SIZE,
        ))),
        mid: Mesh2dHandle(meshes.add(Rectangle::new(
            LANE_WIDTH * MAP_SIZE,
            f32::sqrt(2.) * NON_LANE_WIDTH * MAP_SIZE,
        ))),
        lane: Mesh2dHandle(meshes.add(Rectangle::new(LANE_WIDTH * MAP_SIZE, MAP_SIZE))),
        base: Mesh2dHandle(meshes.add(CircularSector::from_radians(
            BASE_RADIUS * MAP_SIZE,
            2. * PI / 4.,
        ))),

        //spawner
        spawner: Mesh2dHandle(meshes.add(Circle::new(SPAWNER_RADIUS))),

        //units
        unit: Mesh2dHandle(meshes.add(Circle::new(UNIT_RADIUS))),
        direction: Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::new(UNIT_RADIUS, 0.),
            Vec2::new(
                -UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.cos(),
                UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.sin(),
            ),
            Vec2::new(
                -UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.cos(),
                -UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.sin(),
            ),
        ))),
    });
}

fn add_assets(
    mut commands: Commands,
    handles: Res<Handles>,
    map_query: Query<(), With<Map>>,
    spawner_query: Query<(), With<Spawner>>,
    unit_query: Query<&Team, With<Unit>>,
    mut ev_graphics: EventReader<GraphicsEvent>,
) {
    for ev in ev_graphics.read() {
        let entity = commands.get_entity(ev.entity).unwrap();
        if let Ok(_) = map_query.get(ev.entity) {
            add_map(entity, &handles);
        } else if let Ok(_) = spawner_query.get(ev.entity) {
            add_spawner(entity, &handles);
        } else if let Ok(team) = unit_query.get(ev.entity) {
            add_unit(entity, &handles, *team);
        }
    }
}

fn add_map(mut entity: EntityCommands, handles: &Res<Handles>) {
    let mut children: Vec<Entity> = Vec::new();
    let mut commands = entity.commands();
    //plain
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.plain.clone(),
                material: handles.green.clone(),
                transform: vec4_to_trans(Vec4::new(0., 0., -6., 0.)),
                ..default()
            })
            .id(),
    );
    //river
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.river.clone(),
                material: handles.teal.clone(),
                transform: vec4_to_trans(Vec4::new(0., 0., -5., PI / 4.)),
                ..default()
            })
            .id(),
    );
    //mid
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.mid.clone(),
                material: handles.yellow.clone(),
                transform: vec4_to_trans(Vec4::new(0., 0., -4., -PI / 4.)),
                ..default()
            })
            .id(),
    );
    //blue top
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.lane.clone(),
                material: handles.yellow.clone(),
                transform: vec4_to_trans(Vec4::new(0., MID_LANE * MAP_SIZE, -3., 2. * PI / 4.)),
                ..default()
            })
            .id(),
    );
    //red top
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.lane.clone(),
                material: handles.yellow.clone(),
                transform: vec4_to_trans(Vec4::new(-MID_LANE * MAP_SIZE, 0., -3., 0.)),
                ..default()
            })
            .id(),
    );
    //red bot
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.lane.clone(),
                material: handles.yellow.clone(),
                transform: vec4_to_trans(Vec4::new(0., -MID_LANE * MAP_SIZE, -3., 2. * PI / 4.)),
                ..default()
            })
            .id(),
    );
    //blue bot
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.lane.clone(),
                material: handles.yellow.clone(),
                transform: vec4_to_trans(Vec4::new(MID_LANE * MAP_SIZE, 0., -3., 0.)),
                ..default()
            })
            .id(),
    );
    //red base
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.base.clone(),
                material: handles.red.clone(),
                transform: vec4_to_trans(Vec4::new(-MAP_SIZE / 2., -MAP_SIZE / 2., -2., -PI / 4.)),
                ..default()
            })
            .id(),
    );
    //blue base
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.base.clone(),
                material: handles.blue.clone(),
                transform: vec4_to_trans(Vec4::new(
                    MAP_SIZE / 2.,
                    MAP_SIZE / 2.,
                    -2.,
                    3. * PI / 4.,
                )),
                ..default()
            })
            .id(),
    );
    for child in children {
        entity.add_child(child);
    }
}

fn add_spawner(mut entity: EntityCommands, handles: &Res<Handles>) {
    let mut children: Vec<Entity> = Vec::new();
    let mut commands = entity.commands();
    //purple circle
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.spawner.clone(),
                material: handles.purple.clone(),
                transform: vec4_to_trans(Vec4::new(0., 0., 0., 0.)),
                ..default()
            })
            .id(),
    );
    for child in children {
        entity.add_child(child);
    }
}

fn add_unit(mut entity: EntityCommands, handles: &Res<Handles>, team: Team) {
    let mut children: Vec<Entity> = Vec::new();
    let mut commands = entity.commands();
    //green circle
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.unit.clone(),
                material: handles.green.clone(),
                transform: vec4_to_trans(Vec4::new(0., 0., 0., 0.)),
                ..default()
            })
            .id(),
    );
    //red/blue triangle
    children.push(
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: handles.direction.clone(),
                material: match team {
                    Team::Red => handles.red.clone(),
                    Team::Blue => handles.blue.clone(),
                },
                transform: vec4_to_trans(Vec4::new(0., 0., 1., 0.)),
                ..default()
            })
            .id(),
    );
    for child in children {
        entity.add_child(child);
    }
}
