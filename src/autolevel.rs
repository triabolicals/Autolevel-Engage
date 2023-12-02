use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};

use engage::{gamevariable::*, gameuserdata::*};
use crate::engage_functions::*;
pub static mut INITIAL_LEVEL : [u8; 950] = [0; 950];
pub static mut CLASS_LEVEL : [u8; 950] = [0; 950]; // 1 - 10 - unpromoted, 20 - promoted, - 3 special
pub static mut LEVEL_SET: bool = false;

// if person has combat bgm then it's a boss
pub fn is_boss(this: &PersonData) -> bool {
    unsafe { 
        let bgm = person_get_combat_bgm(this, None);
        return !is_null_empty(bgm, None);
    }
}

pub fn increaseGrow(this: &PersonData, amount: u8){
    unsafe { 
        let grow = get_Grow(this, None);
        for i in 0..10 {
            if i == 8 {
                let half = (amount/2 ) as u8;
                Capability_add(grow, i, half, None);
            }
            Capability_add(grow, i, amount, None);
        }
        set_grow(this, grow, None);
    }
}
pub fn increaseCaps(this: &PersonData, amount : u8){
    unsafe {
        let caps = get_limit(this, None);
        for i in 0..10 {
            if i == 8 { continue; }
            else {
                Capability_add(caps, i, amount, None);
            }
        }
        set_limit(this, caps, None);
    }
}

pub fn get_initial_levels() {
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    for x in 1..900 {
        unsafe {
            let level = get_level(t_list[x], None); 
            INITIAL_LEVEL[x] = level; 
            if x == 2 { 
                increaseCaps(t_list[x], 5);
                continue; 
            } // ignore Vander
            if x < 33 || x == 39 || x == 49 || x == 51 || x == 52 { 
                increaseGrow(t_list[x], 10);
                increaseCaps(t_list[x], 5);
            }
            else {
                increaseCaps(t_list[x], 10);
                let grow = get_Grow(t_list[x], None);
                if ( !Capability_is_zero(grow, None)) { increaseGrow(t_list[x], 10); }
            }
            let job = GetJob(t_list[x], None);
            let jid = get_jid(t_list[x], None);
            if is_null_empty(jid, None) { 
                CLASS_LEVEL[x] = 0;
                continue; 
            }
            if job_is_low(job, None) {
                if  job_has_high_job(job, None) {
                    if job_getWeaponSword(job, None) == 1 { CLASS_LEVEL[x] = 1; }
                    else if job_getWeaponLance(job, None) == 1 { CLASS_LEVEL[x] = 2; }
                    else if job_getWeaponAxe(job, None) == 1 { CLASS_LEVEL[x] = 3; }
                    else { CLASS_LEVEL[x] = 4; } //Other Bow (4), Daggers (5), Magic (6), Staff (7) if needed
                }
                else { CLASS_LEVEL[x] = 10; }
            }
            else { CLASS_LEVEL[x] = 20;}
        }
    }
    // DLC Characters
    for x in 0..7 {
        unsafe {
            let level = get_level(t_list[1116+x], None); 
            INITIAL_LEVEL[900+x] = level; 
            increaseGrow(t_list[1116+x], 10);
            CLASS_LEVEL[900+x] = 4;
        }
    }
    unsafe {CLASS_LEVEL[906] = 3;} // Madeline Axe
    unsafe { LEVEL_SET = true; }

    let triabolical2 = JobData::get_list_mut().expect("triabolical2 is 'None'");
    let t_list2 = &triabolical2.list.items;
    unsafe {
    for x in 27..110 {
        let job = &t_list2[x];
        let diff_growL = job_get_DiffGrowL(job, None);
        let diff_growH = job_get_DiffGrowH(job, None);
        for i in 0..9 {
            if i == 8 {
                CapabilitySbyte_add(diff_growL, i, 5, None);
                CapabilitySbyte_add(diff_growH, i, 5, None);
            }
            CapabilitySbyte_add(diff_growL, i, 10, None);
            CapabilitySbyte_add(diff_growH, i, 5, None);
        }
        job_set_DiffGrowH(job, diff_growH, None);
        job_set_DiffGrowL(job, diff_growL, None);
    }
}
}
pub fn promote_person(this: &PersonData, total_level: i32){
    unsafe {
        let job = GetJob(this, None);
        let high_job = job_get_high_job1(job, None);
        person_set_Jid(this, high_job, None);
        let job2 = GetJob(this, None);
        let internal_level = get_job_internal_level(job2, None);
        let mut newLevel = total_level - internal_level as i32;
        set_level(this, newLevel.try_into().unwrap(), None); 
    }
}
pub fn demote_person(this: &PersonData, new_level: i32, weaponType: u8){
    unsafe {
        let job = GetJob(this, None);

        let low_job = job_GetLowJobs(job, None);
        if low_job.len() == 1 { person_set_Jid(this, low_job.items[0].jid, None);  }
        else {
            let selection: usize = (weaponType - 1).into();
            person_set_Jid(this, low_job.items[selection].jid, None);
        }
        set_level(this, new_level.try_into().unwrap(), None); 
    }
}

pub fn auto_level_enemies(this: &PersonData, enemy_level: i32, index: usize){
    unsafe {
        let initial_level = INITIAL_LEVEL[index];
        let class_type = CLASS_LEVEL[index];
        if class_type == 0 { return; }

        let mut total_level = enemy_level;
        if is_boss(this){ total_level = 3 + enemy_level; }
        let current_job = GetJob(this, None);
        // un-promoted case, internal level is assumed to be 0, class max level is assumed to be 20
        if class_type < 10 {
            set_InternalLevel(this, 0, None);
            // if person is not promoted yet
            if job_is_low(current_job, None){
                if i32::from(job_max_level(current_job, None)) < total_level { promote_person(this, total_level); } // Promote them if new level is higher than class max level
                else { set_level(this, total_level.try_into().unwrap(), None); }    // set level
            }
            // If person already promoted
            else {
                let person_total_level = get_level(this, None) + ( get_job_internal_level(current_job, None) as u8 );
                // If current level is too high
                if total_level < person_total_level.into() {
                     // Already promoted but needs to demote (load a previous save, average party level is lower, etc)
                    if total_level <= 20 { 
                        println!("#{} is demoted at level {} with class_type {}", index, total_level, class_type);
                        if total_level < initial_level.into() { demote_person(this, initial_level.into(), class_type); }
                        else { demote_person(this, total_level, class_type); }

                    }
                    else {
                        let mut new_person_level = total_level - 20;
                        // if new level is lower than person's initial level, new level = initial Level
                        //if ( new_person_level < initial_level.into()  ) { new_person_level = initial_level.into() ; }
                        set_level(this, new_person_level.try_into().unwrap(), None);

                    }
                }
                else {
                    let name = get_Name(this, None).get_string().unwrap();
                    let new_person_level = total_level - 20;
                    println!("#{} - {} is at level {}/{}", index, name, new_person_level, total_level);
                    set_level(this, new_person_level.try_into().unwrap(), None);
                }
            }
        }
        //Promoted Units
        //Internal Level is not changed if it not zero 
        else if class_type == 20 {
            let mut person_internal_level = (get_InternalLevel(this, None) as u8 );
            if person_internal_level == 0 { person_internal_level = get_job_internal_level(current_job, None) as u8; }
            let person_total_intial_level = initial_level + person_internal_level; //initial level
            // if total level is less than total initial level then new level is initial level
            if total_level < person_total_intial_level.into() { set_level(this, initial_level.try_into().unwrap(), None); }
            else {
                let new_displayed_level = total_level - person_internal_level as i32;
                set_level(this, new_displayed_level.try_into().unwrap(), None);
            }
        }
        //special units
        else if class_type == 10 {
            if total_level < initial_level.into() { set_level(this, initial_level.try_into().unwrap(), None); }
            else { set_level(this, total_level.try_into().unwrap(), None); }
        }
    }
}

pub fn auto_level_persons(){
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    let diff =  GameUserData::get_difficulty(false);
    let set: bool = unsafe {LEVEL_SET };

    if !(set) { get_initial_levels(); }
    unsafe { 
        //Player_average is exactly Maddening Average - 3
        let mut player_average = GetAverageLevel(2, 14 - 3*diff, None) - 2;
        if player_average < 2 { player_average = 2; }

        let new_enemy_Level = player_average + diff*2;
        println!("Player Army Average Level: {}", player_average);
        println!("NPC Level {}", new_enemy_Level);
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
                if (i32::from(person_total_level) < player_average){ set_level(t_list[x], new_person_level.try_into().unwrap(), None); }
                else { set_level(t_list[x], initial_level.try_into().unwrap(), None);  }
               // println!("Autoleveling {} - {}", x, t_list[x].name.get_string().unwrap());
            }
            //Ignore persons until chapter 5
            else if x > 52 && x < 87 { continue; }
            else { 
                auto_level_enemies(t_list[x], new_enemy_Level, x); 
            }
        }
        // DLC 
        println!("Scaling DLC Characters");
        unsafe {
            if player_average <= 20 {
                for x in 0..7 {
                    let job_dlc = GetJob(t_list[1116+x], None);
                    let class_type = CLASS_LEVEL[900+x];
                    if job_is_low(job_dlc, None){ set_level(t_list[1116+x], player_average.try_into().unwrap(), None); }
                    else { demote_person(t_list[1116+x], player_average.try_into().unwrap(), CLASS_LEVEL[900+x]); }
                }
            }
            else {
                let new_person_level = (player_average as u8) - 20;
                for x in 0..7 {
                    let job_dlc = GetJob(t_list[1116+x], None);
                    let class_type = CLASS_LEVEL[900+x];
                    if job_is_low(job_dlc, None){ 
                        if job_has_high_job(job_dlc, None)  // un-promote now promoted
                            { promote_person(t_list[1116+x], player_average.try_into().unwrap()); }
                            else { set_level(t_list[1116+x], player_average.try_into().unwrap(), None); }
                    }
                    //Already promoted
                    else { set_level(t_list[1116+x], new_person_level.try_into().unwrap(), None); }
                }
            }
        }
    }
}