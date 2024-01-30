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
use cobapi::Event;
use cobapi::SystemEvent;
mod autolevel;
mod engage_functions;
mod dispos;
mod misc;
mod ng;

#[no_mangle]
extern "C" fn load_autolevels(event: &Event<SystemEvent>) {
    if let Event::Args(ev) = event {
        match ev {
            SystemEvent::LanguageChanged => {
                autolevel::get_initial_levels();
            },
            // This syntax means you do not intend to deal with the other events and will do nothing if they are received.
            _ => {println!("Event :D");}
        }
    } 
    else {  println!("We received a missing event, and we don't care!"); }
}
#[skyline::main(name = "Autolevel")]
pub fn main() {
    ng::ng_install();
    misc::auto_install();
    skyline::install_hooks!( ng::set_sid, ng::create_from_dispos, ng::autoGrowCap, misc::join_unit_check, misc::is_recollection, ng::gmap_load, misc::JobLearnSkill);
    skyline::install_hooks!( misc::get_ignots, misc::classChange);
    skyline::install_hooks!( dispos::mapdispos_load);
    println!("Autolevel plugin installed");
    //makes the Average Level displays up to Level 99 instead Level 20/40 
    Patch::in_text(0x0252d124).bytes(&[0x60, 0x0C, 0x80, 0xD2]);
    Patch::in_text(0x02334544).bytes(&[0x01, 0x20, 0x80, 0x52]);
    Patch::in_text(0x01c77620).bytes(&[0xc0, 0x03, 0x5f, 0xd6]);

    Patch::in_text(0x02369530).nop();
    Patch::in_text(0x02369534).nop();

    Patch::in_text(0x01c535e4).bytes(&[0x01,0x7D,0x80,0x52]);
    Patch::in_text(0x01c53670).bytes(&[0x9F, 0xA2, 0x0F, 0xF1]);

    Patch::in_text(0x01a399e0).bytes(&[0xE0, 0x03, 0x1F, 0xAA]);
    Patch::in_text(0x01a399e4).bytes(&[0xc0, 0x03, 0x5f, 0xd6]);

    //Patch::in_text(0x01a0ad00).nop(); //bytes(&[0x00, 0x00, 0x80, 0x52]);
    Patch::in_text(0x01a3a854).bytes(&[0x1F,0x91,0x01,0x71]).unwrap();

    cobapi::register_system_event_handler(load_autolevels);
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
