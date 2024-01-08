use bevy::prelude::{Res, Touches};

pub fn touch_just_pressed() -> impl FnMut(Res<Touches>) -> bool + Clone {
    |inputs: Res<Touches>| inputs.any_just_pressed()
}
