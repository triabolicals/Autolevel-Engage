#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
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
    let add_internal = current_level - this.m_Level;
    if 99 < add_internal + internal_level { this.m_InternalLevel = 99; }
    else { this.m_InternalLevel = add_internal + internal_level; }
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

#[skyline::hook(offset=0x01a08de0)]
pub fn removeVoidCursePlz(this: &mut Unit,  dipos: u64, method_info: OptionalMethod) {
    call_original!(this, dipos, method_info);
    unsafe {
        let void_curse: &str = "SID_虚無の呪い";
        unit_RemoveEquipSkill(this, void_curse.into(), None);
        unit_removeEquipSkillPool(this, void_curse.into(), None);
        this.m_HpStockCount = 1 + this.m_HpStockCount;
    }
}

pub fn updateIgnots(){
    unsafe {
        let instance = GameUserData::get_instance();
        let ironCount = 110 + get_iron(instance, None);
        let steelCount = 11 + get_steel(instance, None);
        let silverCount = 3 + get_silver(instance, None);
        let BondCount = 500 + get_PieceOfBond(instance, None);
        set_PieceOfBond(instance, BondCount, None);
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
                let seed = 4*(ironCount + steelCount + silverCount + BondCount);
                well::set_seed(seed, None);
            }
            else if well::get_well_exchangeLevel(None) < 3 { well::set_well_level(3, None); }
        }
    }
}
//function to load dispos from file
#[skyline::from_offset(0x1cfa150)]
pub fn DisposData_Load(filename: &Il2CppString, method_info: OptionalMethod);

//hooking to load dispos file to autolevel before loading dispos
/*
    Autoleveling Average Party Level with 14 - 3*Difficulty units (Difficulty = 0, 1, 2 for Normal/Hard/Maddening)
    Enemy autolevels with Average Party Level + 2*Difficulty 
    Level Floor is defined in person.xml for the unit
    Player units (unrecruited) set to average party level
*/

#[skyline::hook(offset = 0x029c4120)]
pub fn auto_level_enemies(filename: &Il2CppString, method_info: OptionalMethod){
    unsafe {
        autolevel::auto_level_persons();
        DisposData_Load(filename, None);
    }
}
#[skyline::hook(offset = 0x02513620)]
pub fn get_ignots(this: &GameUserData, method_info: OptionalMethod){
    call_original!(this, None);
    updateIgnots();
}

#[skyline::main(name = "Autolevel")]
pub fn main() {
    skyline::install_hooks!(auto_level_enemies, get_ignots, autoGrowCap, removeVoidCursePlz, autolevel::gmap_load, CreateDLCEnemy, classChange, dispos::mapdispos_load);
    println!("Autolevel plugin installed");
}
