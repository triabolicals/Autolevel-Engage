#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};
use engage::gameuserdata::*;

pub static mut INITIAL_LEVEL : [u8; 900] = [0; 900];
pub static mut LEVEL_SET: bool = false;

//Storing initial levels of persons
pub fn get_initial_levels() {
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    for x in 0..900 {
        
        unsafe {let level = get_level(t_list[x], None); 
         INITIAL_LEVEL[x] = level; 
        }
    }
    unsafe { LEVEL_SET = true; }
}

#[skyline::from_offset(0x2053ea0)]
pub fn get_job_internal_level(this: &JobData, method_info: OptionalMethod) -> u8;

//Check if Il2CppString is empty
#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

//Get Average Level of Party
#[skyline::from_offset(0x2b4afa0)]
pub fn GetAverageLevel(difficulty: i32, sortieCount: i32, method_info: OptionalMethod) -> i32;

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
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    let diff =  GameUserData::get_difficulty(false);
    let set: bool = unsafe {LEVEL_SET };

    if !(set) { get_initial_levels(); }

    unsafe { 
        //Player_average is exactly Maddening Average - 1
        let player_average = GetAverageLevel(2, 14 - 3*diff, None) - 1;
        let new_enemy_Level = player_average + diff*2 + 1;
        println!("Player Army Average Level: {}", player_average);
        println!("NPC set to Level {}", new_enemy_Level);
        for x in 2..751 {

            let initial_level = INITIAL_LEVEL[x];
            //Playable Characters
            if x < 33 || x == 39 || x == 49 || x == 51 || x == 52 {
                let job = GetJob(t_list[x], None);
                let mut person_total_level: u8 = initial_level;
                let mut new_person_level: u8 = 0;
                let person_internal_level = (get_InternalLevel(t_list[x], None) as u8 );
                let internal_level = get_job_internal_level(job, None);
                if person_internal_level == 0 { 
                    person_total_level = internal_level + initial_level; 
                    new_person_level = (player_average as u8) - internal_level;
                }
                else { 
                    person_total_level = person_internal_level  + initial_level; 
                    new_person_level = (player_average as u8) - person_internal_level;
                }
                if new_person_level == 0 { new_person_level = 1; }
                if (i32::from(person_total_level) < player_average){
                    set_level(t_list[x], new_person_level.try_into().unwrap(), None);
                    println!("Playable {}, is set to level {}", t_list[x].name.get_string().unwrap(), new_person_level);
                }
                else {
                    set_level(t_list[x], initial_level.try_into().unwrap(), None); 
                    println!("Playable {}, is set to initial level {}", t_list[x].name.get_string().unwrap(), initial_level);
                }

            }
            else {
                //NPCs - check if NPC is in a class
                let jid = get_jid(t_list[x], None);   
                let no_job = is_null_empty(jid, None);

                if !(no_job){
                    let job = GetJob(t_list[x], None);
                    let internal_level = get_job_internal_level(job, None);
                    let person_total_level = (internal_level as u8) + initial_level;
                    //Change Level if total level is less than set enemy level
                    if (i32::from(person_total_level) < new_enemy_Level){
                        // Subtracting Internal Level for promoted units
                        let mut new_person_level = (new_enemy_Level as u8) - (internal_level as u8);
                        if new_person_level < 1 {new_person_level = 1; }
                        set_level(t_list[x], new_person_level.try_into().unwrap(), None);
                    }
                    else {
                        // set to the original level set in person.xml
                        set_level(t_list[x], initial_level.try_into().unwrap(), None);
                    }
                }
            }
        }
        // Original function did this
        DisposData_Load(filename, None);
    }
}

#[skyline::main(name = "Autolevel")]
pub fn main() {
    skyline::install_hooks!(auto_level_enemies);
    println!("Autolevel plugin installed");
}
