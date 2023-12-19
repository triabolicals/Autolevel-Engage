#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use engage::gamedata::item::ItemData;
use skyline::patching::Patch;
use crate::engage_functions::*;
mod autolevel;
mod engage_functions;
mod dispos;
mod misc;

#[skyline::main(name = "Autolevel")]
pub fn main() {
    skyline::install_hooks!(autolevel::loadtips, autolevel::gmap_load);
    skyline::install_hooks!(misc::create_engrave, misc::ignoreJagens, misc::ignoreMauvierLevel, misc::get_ignots, misc::autoGrowCap, misc::removeVoidCursePlz, misc::CreateDLCEnemy, misc::classChange);
    skyline::install_hooks!(dispos::disposdata_set_pid, dispos::mapdispos_load, dispos::disposdata_set_flag, dispos::unit_set_status);
    println!("Autolevel plugin installed");
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };
        let err_msg = format!(
            "triabolical autolevel plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );
        skyline::error::show_error(
            4,
            "Autolevel has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
}
