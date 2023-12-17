use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};
use engage::{force::*, gamevariable::*, gameuserdata::*, gamedata::unit::*};
use crate::engage_functions::*;

pub static mut INITIAL_LEVEL : [u8; 950] = [0; 950];
pub static mut INITIAL_REC_LEVEL : [u8; 100] = [0; 100];
pub static mut CLASS_LEVEL : [u8; 950] = [0; 950]; // 1 - 10 - unpromoted, 20 - promoted, - 3 special
pub static mut LEVEL_SET: i32 = 0;
pub static mut GROWTH_SET: bool = false;
pub const NG_KEY: &str = "G_NG";
pub const DLC: &[&str] = &["PID_エル", "PID_ラファール", "PID_セレスティア", "PID_グレゴリー", "PID_マデリーン" ];

//Reset World Map and autolevels player units for NG+ when Chapter 26 is completed
pub fn resetGmap(){
    GameVariableManager::make_entry_norewind(NG_KEY, 0);
    let completedGame = GameVariableManager::get_bool("G_Cleared_M026".into());
    if completedGame {
        GameVariableManager::set_bool( "G_Cleared_M006".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M007".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M008".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M009".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M010".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M011".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M012".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M013".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M014".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M015".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M016".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M017".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M018".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M019".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M020".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M021".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M022".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M023".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M024".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M025".into(), false);
        GameVariableManager::set_bool( "G_Cleared_M026".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S003".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S004".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S005".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S006".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S007".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S008".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S009".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S010".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S011".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S012".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S013".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S014".into(), false);
        GameVariableManager::set_bool( "G_Cleared_S015".into(), false);

        GameVariableManager::set_number( "G_GmapSpot_M005".into(), 3);
        GameVariableManager::set_number( "G_GmapSpot_M006".into(), 3);
        GameVariableManager::set_number( "G_GmapSpot_M007".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M008".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M009".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M010".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M011".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M012".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M013".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M014".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M015".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M016".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M017".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M018".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M019".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M020".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M021M022".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M022".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M023".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M024".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_M025".into(), 1);

        GameVariableManager::set_number( "G_GmapSpot_S003".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S004".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S005".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S006".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S007".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S008".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S009".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S010".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S011".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S012".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S013".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S014".into(), 1);
        GameVariableManager::set_number( "G_GmapSpot_S015".into(), 1);
        GameVariableManager::set_bool(NG_KEY, true);
        auto_level_persons();
        unsafe {
            let benchForce = Force_Get(3, None);
            let player_average = GetAverageLevel(2, 10, None) - 2;
            let mut force_iter = Force::iter(benchForce);
            println!("Army 10 Unit Average Level: {}", player_average);
            while let Some(unit) = force_iter.next() {
                let total_level: i32 = (unit.m_Level + unit.m_InternalLevel) as i32;
                let number_of_levelups = player_average - total_level;
    
                if number_of_levelups > 0 {
                    for x in 0..number_of_levelups { Unit_LevelUP(unit, 2, None); }
                    let SP: i32 = 100*number_of_levelups;
                    unit_add_SP(unit, SP, None);
                    println!("Bench Unit {}, gained {} levels up to {}", unit.person.name.get_string().unwrap(), number_of_levelups, unit.m_Level);
                    unit_set_exp(unit, 0, None);
                    let jobmaxLevel = unit.m_Job.MaxLevel;
                    let unit_internal = unit.m_InternalLevel;
                    if jobmaxLevel < unit.m_Level {
                        let excessLevel = unit.m_Level - jobmaxLevel;
                        unit.set_internal_level((unit_internal + excessLevel).into());
                        unit_set_level(unit, jobmaxLevel.into(), None);
                        println!("{} is now Level {}/{}", unit.person.name.get_string().unwrap(),jobmaxLevel, unit_internal + excessLevel);
                    }
                }
            }
        }
    }
}

pub fn is_boss(this: &PersonData) -> bool {
    unsafe { 
        let bgm = person_get_combat_bgm(this, None);
        return !is_null_empty(bgm, None);
    }
}

#[skyline::hook(offset=0x02b3a3f0)]
pub fn gmap_load(this: &u64, method_info: OptionalMethod){
    call_original!(this, method_info);
    resetGmap();
    auto_level_persons();
}

//update "recommended level" to player average
pub fn update_recommendedLevel(){
    let chapters = ChapterData::get_list_mut().expect(":D");
    unsafe {
        let length = chapters.len();
        let diff =  GameUserData::get_difficulty(false);
        let mut player_average = GetAverageLevel(2, 14 - 3*diff, None) - 2;
        if player_average < 2 { player_average = 2; }
        let CID_M: &str = "CID_M";
        let CID_S: &str = "CID_S";
        let CID_M2: &str = "CID_M021";
        for x in 0..length {
            let is_main = str_start_with(chapters[x].cid, CID_M);
            let is_side = str_start_with(chapters[x].cid, CID_S);
            let intial_level = INITIAL_REC_LEVEL[x];
            if is_main || is_side {
                if INITIAL_REC_LEVEL[x] < player_average.try_into().unwrap() { chapter_set_recommended_level(chapters[x], player_average.try_into().unwrap(), None); }
                else { chapter_set_recommended_level(chapters[x], intial_level, None); }
            }
            if str_start_with(chapters[x].cid, CID_M2) { chapter_set_flag(chapters[x], 131, None); }
        }
    }
}

pub fn increaseGrow(this: &PersonData, amount: u8, player: bool){
    unsafe { 
        let grow = get_Grow(this, None);
        for i in 0..9 {
            if i == 4 && !player {continue; }
            if i == 8 {
                let half = (amount/2 ) as u8;
                Capability_add(grow, i, half, None);
            }
            Capability_add(grow, i, amount, None);
        }
        set_grow(this, grow, None);
    }
}
pub fn increaseCaps(this: &PersonData, amount : i8){
    unsafe {
        let caps = get_limit(this, None);
        for i in 0..10 {
            if i == 8 { continue; } // ignore caps for build
            else { CapabilitySbyte_add(caps, i, amount, None); }
        }
        set_limit(this, caps, None);
    }
}
pub fn get_initial_levels() {
    //Only set it if Chapter 4 is complete
    unsafe { 
        let chapters = ChapterData::get_list_mut().expect(":D");
        let length = chapters.len();
        let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
        let t_list = &triabolical.list.items;
        let triabolical2 = JobData::get_list_mut().expect("triabolical2 is 'None'");
        let t_list2 = &triabolical2.list.items;
        //increase growths by 15
        if !GROWTH_SET {
            println!("Getting initial levels and increasing growths");
            for x in 0..length {
                let rec = chapter_get_recommended_level(chapters[x], None);
                INITIAL_REC_LEVEL[x] = rec;
            }
            for x in 1..900 {
                let level = get_level(t_list[x], None); 
                INITIAL_LEVEL[x] = level; 
                let assetForce = person_get_AssetForce(t_list[x], None);
                if x == 2 || get_Pid(t_list[x], None).get_string().unwrap() == "PID_モーヴ" { continue; }
                else if assetForce == 0 { 
                    increaseGrow(t_list[x], 15, true); 
                    println!("Person #{} - {} has their growths increased by 15",x, t_list[x].name.get_string().unwrap());
                    if x == 55 { increaseGrow(t_list[x], 50, true); }
                }
                else {
                    if ( !Capability_is_zero(get_Grow(t_list[x], None), None)) { 
                        println!("Person #{} - {} has their growths increased by 15",x, t_list[x].name.get_string().unwrap());
                        increaseGrow(t_list[x], 15, true); 
                    } 
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
            println!("Getting initial levels for DLC Characters and increasing their growths..");
            for x in 0..DLC.len() {
                let person = PersonData::get(DLC[x]);
                match person {
                    Some(p) => {
                        let level = get_level(p, None); 
                        INITIAL_LEVEL[900+x] = level; 
                        increaseGrow(p, 15, true);
                        CLASS_LEVEL[900+x] = 4;
                    },
                    None => {}
                }
            }
            CLASS_LEVEL[904] = 3; //Madeline Axe
            GROWTH_SET = true;
        }
        if LEVEL_SET != 0 && !GameVariableManager::get_bool( "G_Cleared_M004".into() ) {
            let player_average = 1;
            for x in 2..53 {
                let initial_level = INITIAL_LEVEL[x];

                if person_get_AssetForce(t_list[x], None) == 0 {
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
                }
            }
            for x in 88..758 { auto_level_enemies(t_list[x], 1, x); }
            for x in 8..110 {
                if x < 26 && 10 < x { continue; } 
                let job = &t_list2[x];
                let diff_growL = job_get_DiffGrowL(job, None);
                let diff_growH = job_get_DiffGrowH(job, None);
                for i in 0..9 {
                    if i == 8 {
                        CapabilitySbyte_add(diff_growL, i, -5, None);
                        CapabilitySbyte_add(diff_growH, i, -5, None);
                    }
                    else {
                        CapabilitySbyte_add(diff_growL, i, -10, None);
                        CapabilitySbyte_add(diff_growH, i, -5, None);
                    }
                }
                job_set_DiffGrowL(job, diff_growL, None);
                job_set_DiffGrowH(job, diff_growH, None);
            }
            LEVEL_SET = 0;
            println!("Reset Enemy Class Increase and Playable characters are set to their default level");
            return; 
        }
        //Initialize levels and increase growths, modify generic class growths
        if LEVEL_SET == 0 && GameVariableManager::get_bool( "G_Cleared_M004".into() ) {
            for x in 8..110 {
                if x < 26 && 10 < x { continue; } 
                let job = &t_list2[x];
                let diff_growL = job_get_DiffGrowL(job, None);
                let diff_growH = job_get_DiffGrowH(job, None);
                for i in 0..9 {
                    if i == 8 {
                        CapabilitySbyte_add(diff_growL, i, 5, None);
                        CapabilitySbyte_add(diff_growH, i, 5, None);
                    }
                    else {
                        CapabilitySbyte_add(diff_growL, i, 10, None);
                        CapabilitySbyte_add(diff_growH, i, 5, None);
                    }
                }
                job_set_DiffGrowH(job, diff_growH, None);
                job_set_DiffGrowL(job, diff_growL, None);
            }
            println!("Boosting Enemy Growths");
        }
        if !GameVariableManager::get_bool( "G_Cleared_M004".into() ) { return; }
        let is_NG = GameVariableManager::get_bool(NG_KEY);
        let mut player_cap_increase: i8 = 0;
        let mut npc_cap_increase: i8 = 0;

        let mut genericL_increase: i8 = 0;
        let mut genericH_increase: i8 = 0;
        
        if LEVEL_SET == 0 && is_NG {
            LEVEL_SET = 2;

            player_cap_increase = 45;
            npc_cap_increase = 50;
            println!("Setting mode to NG+");
        //    genericL_increase = 5;
         //   genericH_increase = 5;
        }
        else if LEVEL_SET == 2 && !is_NG {
            LEVEL_SET == 1;

            player_cap_increase = -35;
            npc_cap_increase = -35;
            println!("Setting mode to NG from NG+");
          //  genericL_increase = -5;
          //  genericH_increase = -5;
        }
        else if LEVEL_SET == 1 && is_NG {
            LEVEL_SET = 2;

            player_cap_increase = 35;
            npc_cap_increase = 35;
            println!("Setting mode to NG+ from NG");
         //   genericL_increase = 5;
         //   genericH_increase = 5;
        }
        else if LEVEL_SET == 0 && !is_NG {
            LEVEL_SET = 1;
            player_cap_increase = 10;
            npc_cap_increase = 15;
            println!("Setting mode to NG");
          //  genericL_increase = 0;
          //  genericH_increase = 0;
        }
        if npc_cap_increase != 0 && player_cap_increase != 0 {
            for x in 1..900 {
                let assetForce = person_get_AssetForce(t_list[x], None);
                if assetForce == 0 { increaseCaps(t_list[x], player_cap_increase);  }
                else { increaseCaps(t_list[x], npc_cap_increase); }
            }
            for x in 0..DLC.len() {
                let person = PersonData::get(DLC[x]);
                match person {
                    Some(p) => { increaseCaps(p, player_cap_increase);  },
                    None => {}
                }
            }
        }
        if genericL_increase != 0 && genericH_increase != 0 {
            for x in 8..110 {
                if x < 26 && 10 < x { continue; } 
                let job = &t_list2[x];
                let diff_growL = job_get_DiffGrowL(job, None);
                let diff_growH = job_get_DiffGrowH(job, None);
                for i in 0..8 {
                    CapabilitySbyte_add(diff_growL, i, genericL_increase, None);
                    CapabilitySbyte_add(diff_growH, i, genericH_increase, None);
                }
                job_set_DiffGrowL(job, diff_growL, None);
                job_set_DiffGrowH(job, diff_growH, None);
            }
        }
    }
}

pub fn promote_person(this: &PersonData, total_level: i32){
    unsafe {
        let job = GetJob(this, None);
        let job_jid = job.jid.get_string().unwrap();

        // Fliers to Wyvern
        if job_jid == "JID_ソードペガサス" || job_jid == "JID_ランスペガサス" || job_jid == "JID_アクスペガサス" {
            let high_job = job_get_high_job2(job, None);
            if is_null_empty(high_job, None){
                let high_job1 = job_get_high_job1(job, None);
                person_set_Jid(this, high_job, None);
            }
            else { person_set_Jid(this, high_job, None); }
        }
        else { 
            let high_job = job_get_high_job1(job, None);
            person_set_Jid(this, high_job, None);
        }
        let job2 = GetJob(this, None);
        let internal_level = get_job_internal_level(job2, None);
        let mut newLevel = total_level - internal_level as i32;
        set_level(this, newLevel.try_into().unwrap(), None); 
        let uniticon = get_UnitIconID(this, None);
        if uniticon.get_string().unwrap() == "702MorphLC" {
            let new_unit_icon = "702Morph";
            set_UnitIconID(this, new_unit_icon.into(), None);
        }
    }
}
pub fn demote_person(this: &PersonData, new_level: i32, weaponType: u8){
    unsafe {
        let job = GetJob(this, None);
        let low_job = job_GetLowJobs(job, None);
        if low_job.len() == 1 { person_set_Jid(this, low_job.items[0].jid, None);  }
        else if low_job.len() == 0 {} //do nothing?
        else if low_job.len() >= 3 {
            let selection: usize = (weaponType - 1).into();
            person_set_Jid(this, low_job.items[selection].jid, None);
        }
        else {
            person_set_Jid(this, low_job.items[0].jid, None);
        }
        set_level(this, new_level.try_into().unwrap(), None); 
        let uniticon = get_UnitIconID(this, None);
        if uniticon.get_string().unwrap() == "702Morph" {
            let new_unit_icon = "702MorphLC";
            set_UnitIconID(this, new_unit_icon.into(), None);
        }
    }
}
pub fn auto_level_enemies(this: &PersonData, enemy_level: i32, index: usize){
    unsafe {
        let initial_level = INITIAL_LEVEL[index];
        let class_type = CLASS_LEVEL[index];
        if class_type == 0 { return; }
        let mut total_level = enemy_level;
        if is_boss(this){ total_level = 4 + enemy_level; }
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
                      //  println!("#{} is demoted at level {} with class_type {}", index, total_level, class_type);
                        if total_level < initial_level.into() { demote_person(this, initial_level.into(), class_type); }
                        else { demote_person(this, total_level, class_type); }
                    }
                    else {
                        let mut new_person_level = total_level - 20;
                        // if new level is lower than person's initial level, new level = initial Level
                        set_level(this, new_person_level.try_into().unwrap(), None);
                    }
                }
                else {
                    let name = get_Name(this, None).get_string().unwrap();
                    let new_person_level = total_level - 20;
                  //  println!("#{} - {} is at level {}/{}", index, name, new_person_level, total_level);
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
// function that autolevels 
pub fn auto_level_persons(){
    let NG = GameVariableManager::get_bool(NG_KEY);
    //force casual mode
    get_initial_levels(); 
    if !GameVariableManager::get_bool( "G_Cleared_M004".into() ) { return; }
    if NG {
        unsafe {
            let gameMode = GameUserData::get_game_mode();
            if gameMode == GameMode::Classic { GameUserData::set_game_mode(GameMode::Casual); }
        }
    }
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    let diff = GameUserData::get_difficulty(false);
    update_recommendedLevel();
    unsafe { 
        //Player_average is exactly Maddening Average - 3
        let mut player_average = GetAverageLevel(2, 14 - 3*diff, None) - 2;
        if player_average < 1 { player_average = 1; }
        let new_enemy_Level = player_average + diff*2;
        println!("Player Army Average Level: {}", player_average);
        println!("NPC Level {}", new_enemy_Level);
        for x in 2..53 {
            let initial_level = INITIAL_LEVEL[x];
            //Playable Characters
            if person_get_AssetForce(t_list[x], None) == 0 {
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
            }
        }
        for x in 88..758 { auto_level_enemies(t_list[x], new_enemy_Level, x); }
        if player_average <= 20 {
            for x in 0..DLC.len() {
                let person = PersonData::get(DLC[x]);
                match person {
                    Some(p) => {
                        let job_dlc = GetJob(p, None);
                        let class_type = CLASS_LEVEL[900+x];
                        if job_is_low(job_dlc, None){ set_level(p, player_average.try_into().unwrap(), None); }
                        else { demote_person(p, player_average.try_into().unwrap(), CLASS_LEVEL[900+x]); }
                    },
                    None => {}
                }
            }
        }
        else {
            let new_person_level = (player_average as u8) - 20;
            for x in 0..DLC.len() {
                let person = PersonData::get(DLC[x]);
                match person {
                    Some(p) => {
                        let job_dlc = GetJob(p, None);
                        let class_type = CLASS_LEVEL[900+x];
                        if job_is_low(job_dlc, None){ 
                            if job_has_high_job(job_dlc, None)  // un-promote now promoted
                                { promote_person(p, player_average.try_into().unwrap()); }
                                else { set_level(p, player_average.try_into().unwrap(), None); }
                            }
                            //Already promoted
                            else { set_level(p, new_person_level.try_into().unwrap(), None); }
                        },
                    None => {}
                }
            }
        }
    }
}
#[skyline::hook(offset=0x01cd5f30)]
pub fn ignoreJagens(this: u64, unit: &Unit, method_info: OptionalMethod){
    unsafe {
        let pid = unit_get_pid(unit, None);
        if pid.get_string().unwrap() == "PID_モーヴ" {
           // println!("Mauvier level was not added to the list of levels");
            return;
        }
        else { call_original!(this, unit, method_info); }
    }
}
#[skyline::hook(offset=0x01cd6020)]
pub fn ignoreMauvierLevel(this: u64, unit: &Unit, method_info: OptionalMethod){
    unsafe {
        let pid = unit_get_pid(unit, None);
        if pid.get_string().unwrap() == "PID_モーヴ" {
           // println!("Skipping adding Mauvier's Level");
            return;
        }
        else { call_original!(this, unit, method_info); }
    }
}

//does initial levels upon setting up loading screen tips
#[skyline::hook(offset=0x01becdf0)]
pub fn loadtips(this: u64, tips: u64, method_info: OptionalMethod){
    get_initial_levels();
    call_original!(this, tips, method_info);
} 