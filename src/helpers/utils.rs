use crate::helpers::types::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn spawn_unit(commands: &mut Commands, handles: &Res<Handles>, unit: UnitBundle) {
    let team = unit.team; //avoid borrow checking issue
    commands.spawn(unit).with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: handles.unit.clone(), //cloning handles to resources is safe
            material: handles.green.clone(),
            //visibility: Visibility::Hidden, //hide for now
            ..default()
        });
        parent.spawn(MaterialMesh2dBundle {
            mesh: handles.direction.clone(),
            material: if team == Team::Red {
                handles.red.clone()
            } else {
                handles.blue.clone()
            },
            transform: Transform::from_translation(Vec2::ZERO.extend(1.)), //ensure triangles are rendered above circles
            ..default()
        });
    });
}

pub fn spawn_spawner(commands: &mut Commands, handles: &Res<Handles>, spawner: SpawnerBundle) {
    commands.spawn(spawner).with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: handles.spawner.clone(), //cloning handles to resources is safe
            material: handles.purple.clone(),
            ..default()
        });
    });
}
