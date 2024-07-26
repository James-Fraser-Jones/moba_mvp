use bevy::prelude::*;

//helper functions to convert between transforms and vec4s that store z-rotation in w

pub fn vec4_to_trans(vec4: Vec4) -> Transform {
    Transform::from_translation(vec4.truncate()).with_rotation(Quat::from_rotation_z(vec4.w))
}

pub fn trans_to_vec4(trans: &Transform) -> Vec4 {
    trans
        .translation
        .extend(trans.rotation.to_euler(EulerRot::XYZ).2)
}
