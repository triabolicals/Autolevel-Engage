#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};
use engage::{gamevariable::*, gameuserdata::*};
mod autolevel;
mod engage_functions;

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
            let can_get_items = engage_functions::get_IsItemReturn(None);
            if can_get_items == false {
                engage_functions::set_well_level(2, None);
                engage_functions::set_well_flag(2, None);
                let seed = ironCount + steelCount + silverCount + BondCount;
                engage_functions::set_seed(seed, None);
            }
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
    skyline::install_hooks!(auto_level_enemies, get_ignots);
    println!("Autolevel plugin installed");
}
