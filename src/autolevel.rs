use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};

use engage::{gamevariable::*, gameuserdata::*};
use crate::engage_functions::*;
pub static mut INITIAL_LEVEL : [u8; 950] = [0; 950];
pub static mut CLASS_LEVEL : [u8; 950] = [0; 950]; // 1 - unpromoted, 2 - promoted, - 3 special
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
            if i == 8{
                let half = (amount/2 ) as u8;
                Capability_add(grow, i, half, None);
            }
            Capability_add(grow, i, amount, None);
        }
        set_grow(this, grow, None);
    }
}

pub fn get_initial_levels() {
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    for x in 1..900 {
        unsafe {
            let level = get_level(t_list[x], None); 
            INITIAL_LEVEL[x] = level; 
            if x == 2 { continue; } // ignore Vander
            if x < 33 || x == 39 || x == 49 || x == 51 || x == 52 { increaseGrow(t_list[x], 10); }
            else {
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
                if  job_has_high_job(job, None) { CLASS_LEVEL[x] = 1; }
                else { CLASS_LEVEL[x] = 3; }
            }
            else { CLASS_LEVEL[x] = 2;}
        }
    }
    // DLC Characters
    for x in 0..7 {
        unsafe {
            let level = get_level(t_list[1116+x], None); 
            INITIAL_LEVEL[900+x] = level; 
            increaseGrow(t_list[1116+x], 10);
            let job = GetJob(t_list[x], None);
            let jid = get_jid(t_list[x], None);
            if is_null_empty(jid, None) { 
                CLASS_LEVEL[900+x] = 0;
                continue; 
            }
            if job_is_low(job, None) {
                if  job_has_high_job(job, None) { CLASS_LEVEL[900+x] = 1; }
                else { CLASS_LEVEL[900+x] = 3; }
            }
            else { CLASS_LEVEL[x] = 2;}
        }
    }
    unsafe { LEVEL_SET = true; }
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
pub fn demote_person(this: &PersonData, new_level: i32){
    unsafe {
        let job = GetJob(this, None);
        let low_job = job_get_low(job, None);
        person_set_Jid(this, low_job, None);
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
        if class_type == 1 {
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
                        
                        demote_person(this, total_level);
                        let name = get_Name(this, None).get_string().unwrap();
                        println!("#{} - {} is demoted at level {}", index, name, total_level);
                    }
                    else {
                        let mut new_person_level = total_level - 20;
                        // if new level is lower than person's initial level, new level = initial Level
                        if ( new_person_level < initial_level.into()  ) { new_person_level = initial_level.into() ; }
                        set_level(this, new_person_level.try_into().unwrap(), None);
                    }
                }
                else {
                    let new_person_level = total_level - 20;
                    set_level(this, new_person_level.try_into().unwrap(), None);
                }
            }
        }
        //Promoted Units
        //Internal Level is not changed if it not zero 
        else if class_type == 2 {
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
        else {
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
               // println!("Auto leveling {}", x);
                auto_level_enemies(t_list[x], new_enemy_Level, x); 
               // println!("Finished #{} - {}", x, t_list[x].name.get_string().unwrap());
            }
        }
        // DLC 
        unsafe {
        if player_average <= 20 {
            let gregory_low : &str = "JID_マージ";
            let madeline_low: &str = "JID_アクスアーマー";
            person_set_Jid(t_list[1121], gregory_low.into(), None);
            person_set_Jid(t_list[1122], madeline_low.into(), None);

            set_level(t_list[1121], player_average.try_into().unwrap(), None);
            set_level(t_list[1122], player_average.try_into().unwrap(), None);
            set_level(t_list[1116], player_average.try_into().unwrap(), None);
            set_level(t_list[1118], player_average.try_into().unwrap(), None);
            set_level(t_list[1120], player_average.try_into().unwrap(), None);
        }
        else {
            let gregory_high : &str = "JID_セイジ";
            let madeline_high: &str = "JID_ジェネラル";
            person_set_Jid(t_list[1121], gregory_high.into(), None);
            person_set_Jid(t_list[1122], madeline_high.into(), None);
            let new_person_level = (player_average as u8) - 20;
            set_level(t_list[1121], new_person_level.try_into().unwrap(), None);
            set_level(t_list[1122], new_person_level.try_into().unwrap(), None);

            set_level(t_list[1116], player_average.try_into().unwrap(), None);
            set_level(t_list[1118], player_average.try_into().unwrap(), None);
            set_level(t_list[1120], player_average.try_into().unwrap(), None);
        }
        }
    }
}