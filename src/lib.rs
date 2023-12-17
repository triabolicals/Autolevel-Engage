#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use skyline::patching::Patch;
use crate::engage_functions::*;
mod autolevel;
mod engage_functions;
mod well;
mod dispos;

//Bypassing the internal level limit in the unit$$class change until Level 99
#[skyline::hook(offset=0x01a3c7b0)]
pub fn classChange(this: & mut Unit, job: &JobData, item: &u8, method_info: OptionalMethod){
    let internal_level = this.m_InternalLevel;
    let current_level = this.m_Level;

    call_original!(this, job, item, method_info);
    //for second sealing promoted into unique to prevent internal level to 99
    if (this.m_Level == 21 && current_level < 21) { return; }

    let add_internal = current_level - this.m_Level;
    if 99 < add_internal + internal_level { this.m_InternalLevel = 99; }
    else { this.m_InternalLevel = add_internal + internal_level; }
}
//NG+ bypassing Engrave limit
#[skyline::hook(offset=0x0295ce30)]
pub fn create_engrave(this: u64, method_info: OptionalMethod){
    let NG = GameVariableManager::get_bool("G_NG");
    if NG { Patch::in_text(0x0295d5c8).bytes([0x00, 0x00, 0x80, 0xD2]).unwrap(); }
    else { Patch::in_text(0x0295d5c8).bytes([0xB2, 0xE1, 0xD8, 0x97]).unwrap(); }
    call_original!(this, method_info);
}

#[skyline::from_offset(0x1a36e80)]
pub fn unit_RemoveEquipSkill(this: &Unit, sid: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x01a38ab0)]
pub fn unit_removeEquipSkillPool(this: &Unit, sid: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x01a35df0)]
pub fn unit_has_equipped_skill(this: &Unit, sid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01a378b0)]
pub fn unit_has_private_skill(this: &Unit, sid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01a38090)]
pub fn unit_removePrivateSkill(this: &Unit, sid: &Il2CppString, method_info: OptionalMethod) -> bool;


//Removing Void Curse pt 1
#[skyline::hook(offset = 0x01a08de0)]
pub fn CreateDLCEnemy(this: &Unit, map_dispos: &u64, method_info: OptionalMethod){
    call_original!(this, map_dispos, method_info);
    unsafe {
        let void_curse: &str = "SID_虚無の呪い";
        let hasVoidCurse = unit_has_equipped_skill(this, void_curse.into(), None);
            unit_RemoveEquipSkill(this, void_curse.into(), None);
            unit_removeEquipSkillPool(this, void_curse.into(), None);
            unit_removePrivateSkill(this, void_curse.into(), None);
    }
}

//To force level change if level is greater than 20 for non player related units
#[skyline::hook(offset = 0x01a0b1b0)]
pub fn autoGrowCap(this: &Unit, level: i32, target_level: i32, method_info: OptionalMethod){
    call_original!(this, level, target_level, method_info);

    unsafe {
        let asset_Force = engage_functions::person_get_AssetForce(this.person, None);
        if asset_Force != 0 { unit::unit_set_level(this, level, None);  } 
        else {
            let void_curse: &str = "SID_虚無の呪い";
            unit_RemoveEquipSkill(this, void_curse.into(), None);
            unit_removeEquipSkillPool(this, void_curse.into(), None);
            unit_removePrivateSkill(this, void_curse.into(), None);
        }
    }
}
// Attempt to remove void curse pt 2
#[skyline::hook(offset=0x01a08de0)]
pub fn removeVoidCursePlz(this: &mut Unit,  dipos: u64, method_info: OptionalMethod) {
    call_original!(this, dipos, method_info);
    unsafe {
        let void_curse: &str = "SID_虚無の呪い";
        unit_RemoveEquipSkill(this, void_curse.into(), None);
        unit_removeEquipSkillPool(this, void_curse.into(), None);
        //this.m_HpStockCount = 1 + this.m_HpStockCount;
    }
}

//Auto Ignots
pub fn updateIgnots(){
    unsafe {
        let instance = GameUserData::get_instance();
        let ironCount = 110 + get_iron(instance, None);
        let steelCount = 11 + get_steel(instance, None);
        let silverCount = 3 + get_silver(instance, None);
        let NG = GameVariableManager::get_bool("G_NG");
        if NG {
            let BondCount = 1500 + get_PieceOfBond(instance, None);
            set_PieceOfBond(instance, BondCount, None);
        }
        else {
            let BondCount = 500 + get_PieceOfBond(instance, None);
            set_PieceOfBond(instance, BondCount, None);
        }
        let animal_house_check: &str = "G_拠点_動物小屋通知";
        let r = GameVariableManager::get_bool(animal_house_check);
        let well_check: &str = "G_拠点_裏武器イベント";
        let check2 = GameVariableManager::get_number(well_check);
        if r {
            set_iron(instance, ironCount, None);
            set_steel(instance, steelCount, None);
            set_silver(instance, silverCount, None);
            println!("Ingot: {} Iron, {} Steel, {} Silver", ironCount, steelCount, silverCount);
        }
        if check2 > 1 {
            let can_get_items = well::get_IsItemReturn(None);
            if can_get_items == false {
                well::set_well_level(3, None);
                well::set_well_flag(2, None);
                let seed = 4*(ironCount + steelCount + silverCount + 1750);
                well::set_seed(seed, None);
            }
            else if well::get_well_exchangeLevel(None) < 3 { 
                if NG { well::set_well_level(4, None); }
                else { well::set_well_level(2, None); }
            }
        }
    }
}

//hooking to load dispos file to autolevel before loading dispos
/*
    Autoleveling Average Party Level with 14 - 3*Difficulty units (Difficulty = 0, 1, 2 for Normal/Hard/Maddening)
    Enemy autolevels with Average Party Level + 2*Difficulty 
    Level Floor is defined in person.xml for the unit
    Player units (unrecruited) set to average party level
*/

#[skyline::hook(offset = 0x029c4120)]
pub fn auto_level_enemies(filename: &Il2CppString, method_info: OptionalMethod){
        autolevel::auto_level_persons();
        call_original!(filename, method_info);
}
#[skyline::hook(offset = 0x02513620)]
pub fn get_ignots(this: &GameUserData, method_info: OptionalMethod){
    call_original!(this, None);
    updateIgnots();
}

#[skyline::main(name = "Autolevel")]
pub fn main() {
    skyline::install_hooks!(autolevel::loadtips, create_engrave, autolevel::ignoreJagens, autolevel::ignoreMauvierLevel, auto_level_enemies, get_ignots, autoGrowCap, removeVoidCursePlz, autolevel::gmap_load, CreateDLCEnemy, classChange, dispos::mapdispos_load);
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
