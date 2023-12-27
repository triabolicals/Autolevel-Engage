#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use engage::gamedata::item::ItemData;
use skyline::patching::Patch;
use crate::engage_functions::*;
use engage::force::Force;
use engage::force::Force2;
mod autolevel;
mod engage_functions;
mod dispos;
mod misc;
mod ng;

#[skyline::hook(offset=0x01a1e8c0)]
pub fn GodData_Flag(this: Option<&UnitCheck>, force: i32, isLast: bool, method_info: OptionalMethod){
    unsafe {
        match this {
            Some(unit) => {
                if unit.person.is_some() && unit.m_Job.is_none() {
                    let person = unit.person.as_ref().unwrap();
                    println!("Person {} is being moved to force 3 instead of force 7", person.name.get_string().unwrap());
                    unit_set_job(unit, GetJob(person, None), None);
                    call_original!(this, 3, isLast, method_info);
                    return;
                }
            }
            None => {},
        }
    }
    call_original!(this, force, isLast, method_info);
}

#[skyline::main(name = "Autolevel")]
pub fn main() {
    ng::ng_install();
    misc::auto_install();
    skyline::install_hooks!(misc::join_unit_check, misc::is_recollection, GodData_Flag, autolevel::loadtips, ng::gmap_load, misc::JobLearnSkill);
    skyline::install_hooks!(misc::create_engrave, misc::ignoreJagens, misc::ignoreMauvierLevel, misc::get_ignots, misc::autoGrowCap, misc::classChange);
    skyline::install_hooks!( dispos::disposdata_set_pid, dispos::mapdispos_load, dispos::disposdata_set_flag, dispos::unit_set_status);
    println!("Autolevel plugin installed");
    //makes the Average Level displays up to Level 99 instead Level 20/40 
    Patch::in_text(0x0252d124).bytes(&[0x60, 0x0C, 0x80, 0xD2]);
    // Emblem Alear is a normal ring Patch::in_text(0x0232e518).bytes(&[0x41, 0x00, 0x80, 0x52]);
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
