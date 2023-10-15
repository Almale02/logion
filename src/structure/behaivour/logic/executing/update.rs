use bevy::prelude::*;

use crate::{
    structure::behaivour::{
        data::data_types::{hardware::text_display::SBTextDisplay},
        logic::lib::sb_script::SBScript,
    },
};

use super::lib::SBExecutionFrame;

pub fn exec_scrpits(world: &mut World) {
    let mut q_scripts = world.query::<(&mut SBScript, Entity)>();

    let world: *mut World = &mut *world;

    unsafe {
        for mut script in q_scripts.iter_mut(&mut *world) {
            for (idx, data) in script.0.exec_queue.clone().iter().enumerate() {
                let mut exec_frame = SBExecutionFrame::new(script.1);

                script.0.exec_queue.remove(idx);

                dbg!(exec_frame
                    .execute_frame(&mut *world, data.clone())
                    .0
                    .get("self".into())
                    .unwrap()
                    .downcast_ref::<SBTextDisplay>());
            }
        }
    }
}
