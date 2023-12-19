use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use skyline::patching::Patch;
use crate::engage_functions::*;
// Misc things

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
        let asset_Force = person_get_AssetForce(this.person, None);
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
    }
}

//When Chapter is completed to get ignots and well
#[skyline::hook(offset = 0x02513620)]
pub fn get_ignots(this: &GameUserData, method_info: OptionalMethod){
    call_original!(this, None);
    updateIgnots();
}

//Auto Ignots and well
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
            let can_get_items = get_IsItemReturn(None);
            if can_get_items == false {
                set_well_level(3, None);
                set_well_flag(2, None);
                let seed = 4*(ironCount + steelCount + silverCount + 1750);
                set_seed(seed, None);
            }
            else if get_well_exchangeLevel(None) < 3 { 
                if NG { set_well_level(4, None); }
                else { set_well_level(2, None); }
            }
        }
    }
}
//Ignore Mauvier for Reverse Recruitment
#[skyline::hook(offset=0x01cd5f30)]
pub fn ignoreJagens(this: u64, unit: &Unit, method_info: OptionalMethod){
    unsafe {
        let pid = unit_get_pid(unit, None);
        if pid.get_string().unwrap() == "PID_モーヴ" { return; }
        else { call_original!(this, unit, method_info); }
    }
}
#[skyline::hook(offset=0x01cd6020)]
pub fn ignoreMauvierLevel(this: u64, unit: &Unit, method_info: OptionalMethod){
    unsafe {
        let pid = unit_get_pid(unit, None);
        if pid.get_string().unwrap() == "PID_モーヴ" { return;  }
        else { call_original!(this, unit, method_info); }
    }
}
