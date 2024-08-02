use bevy::prelude::*;

//these should only be used for orienting non-logical child entities such as meshes
pub fn vec4_to_trans(vec4: Vec4) -> Transform {
    Transform::from_translation(vec4.truncate()).with_rotation(Quat::from_rotation_z(vec4.w))
}
