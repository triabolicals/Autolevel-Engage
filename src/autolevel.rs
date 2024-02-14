use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};
use engage::{force::*, gamevariable::*, gameuserdata::*, gamedata::unit::*};
use crate::engage_functions::*;
use crate::ng::reset_units;
use crate::misc::Mess_Get;
use skyline::patching::Patch;

pub static mut INITIAL_LEVEL : [u8; 1000] = [0; 1000];
pub static mut INITIAL_REC_LEVEL : [u8; 200] = [0; 200];
pub static mut CLASS_LEVEL : [u8; 1000] = [0; 1000]; // 1 - 10 - unpromoted, 20 - promoted, - 3 special
pub static mut CLASS_TYPE: [u8; 512] = [0; 512];
pub static mut LEVEL_SET: i32 = 0;
pub static mut FX_start: usize = 0;
pub static mut FX_end: usize = 0;
pub static mut AVERAGE: i32 = 0;
pub static mut GROWTH_SET: bool = false;
pub static mut NO_CAPS: bool = false;
pub const NG_KEY: &str = "G_NG";
pub const DLC: &[&str] = &["PID_エル", "PID_ラファール", "PID_セレスティア", "PID_グレゴリー", "PID_マデリーン" ];

pub fn is_reverse_recruitment() -> bool {
    let person = PersonData::get("PID_ヴェイル");
    unsafe {
        match person {
            Some(p) => { 
                match skillarray_find(get_CommonSkill(p, None), "SID_主人公".into(), None) {
                    Some(i) => { return true; }
                    None => { return false; }
                }
            },
            None => { return false; }
        }
    }
    return false;
}

pub fn check_no_caps() -> bool {
    let triabolical2 = JobData::get_list_mut().expect("triabolical2 is 'None'");
    let t_list2 = &triabolical2.list.items;
    unsafe {
        let mut count = 0;
        for x in 0..20 {
            let cap = job_get_limit(t_list2[x], None);
            for i in 0..10 {
                if cap.array.m_item[i] >= 127 { count += 1;  }
            }
        }
        if count > 100 { 
            NO_CAPS = true; 
            return true;
        }
        else {
            NO_CAPS = false;
            return false;
        }
    }
}


pub fn autolevel_party(average_num: i32, diff_from_average: i32, limit :bool ){
    // Autolevels force 3
    // Diff from average # is below the numUnit average
    // limit filters all units within 5 gets autoleveled
    let mut number = 10;
    if average_num > 0 { number = average_num; }
    unsafe {
        let benchForce = Force_Get(3, None);
        let player_average = GetAverageLevel(2, number, None) - diff_from_average;
        let mut force_iter = Force::iter(benchForce);
        while let Some(unit) = force_iter.next() {
            let total_level: i32 = (unit.m_Level as i8 + unit.m_InternalLevel) as i32;
            let number_of_levelups = player_average - total_level;
            if limit && number_of_levelups > 5 { continue; }
            multipleLevelUps(unit, number_of_levelups);
        }
    }
}
pub fn autolevel_party_to_level(level: i32){
    // Autolevels force 3 to level 
    unsafe {
        let benchForce = Force_Get(3, None);
        let mut force_iter = Force::iter(benchForce);
        while let Some(unit) = force_iter.next() {
            let total_level: i32 = (unit.m_Level as i8 + unit.m_InternalLevel) as i32;
            let number_of_levelups = level - total_level;
            if number_of_levelups > 0 {
                multipleLevelUps(unit, number_of_levelups);
            }
        }
    }
}

#[skyline::from_offset(0x01a36560)]
pub fn Add_to_Equip_Skill_Pool(this: &Unit, skill: &SkillData, method_info: OptionalMethod);

#[unity::from_offset("App","Unit", "SetWeaponMaskFromParson")]
pub fn set_weapon_mask_from_person(this: &Unit, method_info: OptionalMethod);

pub fn multipleLevelUps(unit: &Unit, numberOfLevels: i32){
    // Levels up unit and fixes their HP and internal Level
    if numberOfLevels < 0 { return; }
    unsafe {
        let previousHP = unit_get_Hp(unit, None);
        let HP_capability = unit_get_capability(unit, 0, false, None);
        for x in 0..numberOfLevels { 
            Unit_LevelUP(unit, 4, None);
            unit_add_SP(unit, 100, None);
        }
        if random_getMinMax(random_get_Game(None), 0, 100, None) < 25 {
            Unit_LevelUP(unit, 5, None);
            unit_set_level(unit, (unit.m_Level-1).into(), None);
        }
        let new_HP_capability = unit_get_capability(unit, 0, true, None);
        let new_HP = previousHP + new_HP_capability - previousHP;
        unit_set_Hp(unit, new_HP, None);
        let jobmaxLevel = unit.m_Job.MaxLevel;
        let unit_internal = unit.m_InternalLevel;
        if jobmaxLevel < unit.m_Level {
            if job_is_low(unit.m_Job, None) {
                let high_job = job_get_high_job1(unit.m_Job, None);
                if !is_null_empty(high_job, None){
                    let hjob = JobData::get(&high_job.get_string().unwrap());
                    if hjob.is_some(){ 
                        unit.class_change(hjob.unwrap());
                        unit_update_weapon_mask(unit, None);
                        return;
                    }
                }
            }
            let excess_Level = unit.m_Level as i8 - jobmaxLevel as i8;
            unit.set_internal_level((unit_internal + excess_Level).into());
            unit_set_level(unit, jobmaxLevel.into(), None);
        }
        LearnJobSkill_Unit(unit, None);
    }
}

pub fn is_DLC(unit: &Unit) -> bool {
    let pid = unit.person.pid.get_string().unwrap();
    for i in 0..DLC.len() { if pid == DLC[i] {  return true; }  }
    return false;
}

pub fn autolevel_DLC(){
    unsafe {
        let benchForce = Force_Get(3, None);
        let player_average = GetAverageLevel(2, 10, None) - 3;
        let mut force_iter = Force::iter(benchForce);
        while let Some(unit) = force_iter.next() {
            if !is_DLC(unit) { continue; }
            let total_level: i32 = (unit.m_Level as i8 + unit.m_InternalLevel) as i32;
            let number_of_levelups = player_average - total_level;
            multipleLevelUps(unit, number_of_levelups);
        }
    }
}


// To Determine who is a 'Boss' by checking if they have a special BGM
pub fn is_boss(this: &PersonData) -> bool { unsafe { !is_null_empty(person_get_combat_bgm(this, None), None) } }

//update "recommended level" to player average
pub fn update_recommendedLevel(){
    let chapters = ChapterData::get_list_mut().expect(":D");
    unsafe {
        let length = chapters.len();
        let diff =  GameUserData::get_difficulty(false);
        let mut player_average = GetAverageLevel(2, 14 - 2*diff, None) - 3;
        let game_variable = GameUserData::get_variable();
        if player_average < 2 { player_average = 2; }
        for x in 0..length {
            let intial_level = INITIAL_REC_LEVEL[x];
            if str_start_with(chapters[x].cid, "CID_M") || str_start_with(chapters[x].cid, "CID_S") {
                if INITIAL_REC_LEVEL[x] < player_average.try_into().unwrap() { chapter_set_recommended_level(chapters[x], player_average.try_into().unwrap(), None); }
                else { chapter_set_recommended_level(chapters[x], intial_level, None); }
            }
            if str_start_with(chapters[x].cid, "CID_E") {
                chapter_set_HoldLevel(chapters[x], 0, None);
                if get_bool(game_variable, GetClearedFlagName(chapters[x], None), None){
                    if str_start_with(chapters[x].cid, "CID_E004") { chapter_set_flag(chapters[x], 24891, None); }
                    else if str_start_with(chapters[x].cid, "CID_E005") { chapter_set_flag(chapters[x], 49467, None); }
                    else if str_start_with(chapters[x].cid, "CID_E006") { chapter_set_flag(chapters[x], 16659, None); }
                    else { chapter_set_flag(chapters[x], 315, None); }
                }
                else {
                    chapter_set_flag(chapters[x], 313, None);
                    if str_start_with(chapters[x].cid, "CID_E004") { chapter_set_flag(chapters[x], 24889, None); }
                    if str_start_with(chapters[x].cid, "CID_E005") { chapter_set_flag(chapters[x], 49465, None); }
                    if str_start_with(chapters[x].cid, "CID_E006") { chapter_set_flag(chapters[x], 16657, None); }
                }
            }
        }
    }
}
pub fn increaseGrow(this: &PersonData, amount: u8, player: bool){
    unsafe { 
        let grow = get_Grow(this, None);
        for i in 0..9 {
            if ( i == 8 || i == 4 || i == 5 || i == 7) && !player {continue; }
            if i == 8 {
                let half = (amount/2 ) as u8;
                Capability_add(grow, i, half, None);
            }
            else { Capability_add(grow, i, amount, None); }
        }
        set_grow(this, grow, None);
    }
}
//  HP  Str Dex Spd Luck Def Mag Mdef Phys Sight Move
pub fn increaseCaps(this: &PersonData, amount : i8, is_enemy: bool){
    unsafe {
        if NO_CAPS { return; }
        let caps = get_limit(this, None);
        let half = (amount/2 ) as i8;
        if amount == 30 { CapabilitySbyte_add(caps, 0, 30, None); }
        else if amount == -30 {  CapabilitySbyte_add(caps, 0, -30, None); }
        else if is_enemy { CapabilitySbyte_add(caps, 0, amount, None); }
        if is_enemy {
            CapabilitySbyte_add(caps, 1, amount, None);
            if amount == 30 || amount == -30 {
                CapabilitySbyte_add(caps, 2, amount, None);
                CapabilitySbyte_add(caps, 3, amount, None)
            }
            else {
                CapabilitySbyte_add(caps, 2, half, None);
                CapabilitySbyte_add(caps, 3, half, None);
            }

            CapabilitySbyte_add(caps, 5, amount, None);
            CapabilitySbyte_add(caps, 6, amount, None);
            CapabilitySbyte_add(caps, 7, amount, None);
        }
        else {
            if amount == 5 { CapabilitySbyte_add(caps, 4, 2*amount, None); }
            else { CapabilitySbyte_add(caps, 4, amount, None);}
            CapabilitySbyte_add(caps, 2, amount, None);
            CapabilitySbyte_add(caps, 1, amount, None);
            CapabilitySbyte_add(caps, 3, amount, None);
            CapabilitySbyte_add(caps, 5, amount, None);
            CapabilitySbyte_add(caps, 6, amount, None);
            CapabilitySbyte_add(caps, 7, amount, None);
        }
        set_limit(this, caps, None);
    }
}
// Store initial level of units do caps/growths changes
pub fn get_initial_levels() {
    //Only set it if Chapter 4 is complete
    check_no_caps();
    unsafe { 
        let chapters = ChapterData::get_list_mut().expect(":D");
        let length = chapters.len();
        let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
        let t_list = &triabolical.list.items;
        let triabolical2 = JobData::get_list_mut().expect("triabolical2 is 'None'");
        let job_count: usize = JobData::get_count() as usize; 
        let person_count: usize = PersonData::get_count() as usize; 
        if !GROWTH_SET {
            // Fix Stat Caps
            for x in 0..person_count {
                let caps = get_limit(t_list[x], None);
                for i in 0..10 {
                    if NO_CAPS { caps.array.m_item[i] = 0; }
                    else if caps.array.m_item[i] > 5 { caps.array.m_item[i] = 5;}
                } 
                //set_attrs(t_list[x], 524287, None);
                set_limit(t_list[x], caps, None);
            }
            if !NO_CAPS {
                // Fix class caps and adjust class max level to 99 for special and advanced
                for x in 0..job_count {
                    let job = &triabolical2[x];
                    if job_is_low(job, None) && job_max_level(job, None) == 40 {
                        CLASS_TYPE[x] = 2;
                        job_set_maxLevel(job, 100, None);
                    }
                    else if !job_is_low(job, None) && job_max_level(job, None) == 20 { 
                        CLASS_TYPE[x] = 1;
                        job_set_maxLevel(job, 99, None); 
                    }
                    else { CLASS_TYPE[x] = 0; }
                    let cap = job_get_limit(job, None);
                    if cap.array.m_item[0] > 100 { cap.array.m_item[0] = 100; }
                    Capability_add(cap, 0, 10, None);
                    for i in 1..10 {
                        if cap.array.m_item[i] > 60 { cap.array.m_item[i] = 60; }
                        if i != 4 && i != 8 { Capability_add(cap, i.try_into().unwrap(), 20, None); }
                    }
                    job_set_limit(job, cap, None);
                }
            }
            for x in 0..chapters.len() {
                let rec = chapter_get_recommended_level(chapters[x], None);
                INITIAL_REC_LEVEL[x] = rec;
                if str_start_with(chapters[x].cid, "CID_M021") { chapter_set_flag(chapters[x], 131, None); }
                if str_start_with(chapters[x].cid, "CID_E") {
                    chapter_set_HoldLevel(chapters[x], 0, None);
                    chapter_set_flag(chapters[x], 313, None);
                    if str_start_with(chapters[x].cid, "CID_E004") { chapter_set_flag(chapters[x], 24889, None); }
                    if str_start_with(chapters[x].cid, "CID_E005") { chapter_set_flag(chapters[x], 49465, None); }
                    if str_start_with(chapters[x].cid, "CID_E006") { chapter_set_flag(chapters[x], 16657, None); }
                }
            }
            println!("Getting initial levels and increasing growths");
            for x in 1..790 {
                let level = get_level(t_list[x], None); 
                INITIAL_LEVEL[x] = level; 

                let assetForce = person_get_AssetForce(t_list[x], None);
                if assetForce == 0 { 
                    increaseCaps(t_list[x], 5, false);
                    if x == 55 { increaseGrow(t_list[x], 50, true); }
                }
                else {
                    increaseCaps(t_list[x], 10, true);
                    if ( !Capability_is_zero(get_Grow(t_list[x], None), None)) {  increaseGrow(t_list[x], 10, false);  } 
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
            for x in 0..DLC.len() {
                let person = PersonData::get(DLC[x]);
                match person {
                    Some(p) => {
                        let level = get_level(p, None); 
                        INITIAL_LEVEL[790+x] = level; 
                        increaseCaps(p, 5, false);
                        CLASS_LEVEL[790+x] = 4;
                    },
                    None => {}
                }
            }
            CLASS_LEVEL[794] = 3; //Madeline Axe
            //finding index for FX
            FX_start = 0;
            FX_end = 0;
            for x in 1000..1500 {
                if str_start_with(t_list[x].pid, "PID_E00") && FX_start == 0 { FX_start = x; }
                if str_start_with(t_list[x].pid, "PID_E00") { FX_end = x; }
                else {
                    let assetForce = person_get_AssetForce(t_list[x], None);
                    if assetForce != 0 { 
                        increaseCaps(t_list[x], 10, true); 
                        if ( !Capability_is_zero(get_Grow(t_list[x], None), None)) {  increaseGrow(t_list[x], 10, false);  } 
                    }
                }
            }
            for x in 0..(FX_end-FX_start) {
                let index: usize = 800+x;
                let pid_index: usize = (FX_start as usize) +x;
                let job = GetJob(t_list[pid_index], None);
                let jid = get_jid(t_list[pid_index], None);
                if is_null_empty(jid, None) {  CLASS_LEVEL[index] = 0; continue; }
                //if !Capability_is_zero(get_Grow(t_list[pid_index], None), None) { increaseGrow(t_list[pid_index], 10, true); }
                //increaseCaps(t_list[x], 10, true);
                if job_is_low(job, None) {
                    if  job_has_high_job(job, None) {
                        if job_getWeaponSword(job, None) == 1 { CLASS_LEVEL[index] = 1; }
                        else if job_getWeaponLance(job, None) == 1 { CLASS_LEVEL[index] = 2; }
                        else if job_getWeaponAxe(job, None) == 1 { CLASS_LEVEL[index] = 3; }
                        else { CLASS_LEVEL[index] = 4; }
                    }
                    else { CLASS_LEVEL[index] = 10; }
                }
                else { CLASS_LEVEL[index] = 20;}
            }
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
            for x in 8..job_count {
                if x < 26 && 10 < x { continue; } 
                let job = &triabolical2[x]; 
                let diff_growL = job_get_DiffGrowL(job, None);
                let diff_growH = job_get_DiffGrowH(job, None);
                for i in 0..9 {
                    CapabilitySbyte_add(diff_growL, i, -5, None);
                    CapabilitySbyte_add(diff_growH, i, -5, None);
                }
                job_set_DiffGrowL(job, diff_growL, None);
                job_set_DiffGrowH(job, diff_growH, None);
            }
            LEVEL_SET = 0;
            return; 
        }
        //Initialize levels and increase growths, modify generic class growths
        if LEVEL_SET == 0 && GameVariableManager::get_bool( "G_Cleared_M004".into() ) {
            for x in 8..job_count {
                if x < 26 && 10 < x { continue; } 
                let job = &triabolical2[x]; 
                let diff_growL = job_get_DiffGrowL(job, None);
                let diff_growH = job_get_DiffGrowH(job, None);
                for i in 0..8 {
                    CapabilitySbyte_add(diff_growL, i, 5, None);
                    CapabilitySbyte_add(diff_growH, i, 5, None);
                }
                job_set_DiffGrowH(job, diff_growH, None);
                job_set_DiffGrowL(job, diff_growL, None);
            }
        }
        if !GameVariableManager::get_bool( "G_Cleared_M004".into() ) { return; }
        let is_NG = GameVariableManager::get_bool(NG_KEY);
        let mut player_cap_increase: i8 = 0;
        let mut npc_cap_increase: i8 = 0;
        let mut fx_cap_increase: i8 = 0;
        let mut generic_growth: i8 = 0;
        println!("Current mode {}", LEVEL_SET);
        if LEVEL_SET == 0 && is_NG {
            LEVEL_SET = 2;
            player_cap_increase = 30;
            npc_cap_increase = 30;
            fx_cap_increase = 30;
            generic_growth = 5;
            println!("Setting mode to NG+");
        }
        else if LEVEL_SET == 2 && !is_NG {
            LEVEL_SET = 1;
            player_cap_increase = -30;
            npc_cap_increase = -30;
            fx_cap_increase = -30;
            generic_growth = -5;
            println!("Setting mode to NG from NG+");
        }
        else if LEVEL_SET == 1 && is_NG {
            LEVEL_SET = 2;
            player_cap_increase = 30;
            npc_cap_increase = 30;
            fx_cap_increase = 30;
            generic_growth = 5;
            println!("Setting mode to NG+ from NG");
        }
        else if LEVEL_SET == 0 && !is_NG {
            LEVEL_SET = 1;
            player_cap_increase = 0;
            npc_cap_increase = 0;
            fx_cap_increase = 0;
            generic_growth = 0;
            println!("Setting mode to NG");
        }
        if npc_cap_increase != 0 && player_cap_increase != 0 {
            for x in 1..900 {
                if person_get_AssetForce(t_list[x], None) == 0 { increaseCaps(t_list[x], player_cap_increase, false);  }
                else {  increaseCaps(t_list[x], npc_cap_increase, true);  }
            }
            for x in 0..DLC.len() {
                let person = PersonData::get(DLC[x]);
                match person {
                    Some(p) => { increaseCaps(p, player_cap_increase, false);  },
                    None => {}
                }
            }
            for x in 1000..person_count {
                if person_get_AssetForce(t_list[x], None) != 0 { increaseCaps(t_list[x], npc_cap_increase, true); } 
            }
        }
        if generic_growth != 0 {
            for x in 8..job_count {
                if x < 26 && 10 < x { continue; } 
                let job = &triabolical2[x];
                let diff_growL = job_get_DiffGrowL(job, None);
                let diff_growH = job_get_DiffGrowH(job, None);
                for i in 0..8 {
                    if i == 5 || i == 7 { continue; }
                    else if i < 4 || i == 6 { CapabilitySbyte_add(diff_growL, i, 2*generic_growth, None); }
                    else { CapabilitySbyte_add(diff_growL, i, generic_growth, None); }
                    CapabilitySbyte_add(diff_growH, i, generic_growth, None);
                }
                job_set_DiffGrowH(job, diff_growH, None);
                job_set_DiffGrowL(job, diff_growL, None);
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
                person_set_Jid(this, high_job1, None);
            }
            else { person_set_Jid(this, high_job, None); }
        }
        else {  person_set_Jid(this,job_get_high_job1(job, None), None);}
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
        else { person_set_Jid(this, low_job.items[0].jid, None); }
        set_level(this, new_level.try_into().unwrap(), None); 
        let uniticon = get_UnitIconID(this, None);
        if uniticon.get_string().unwrap() == "702Morph" {
            let new_unit_icon = "702MorphLC";
            set_UnitIconID(this, new_unit_icon.into(), None);
        }
    }
}
//Function to autolevel enemies
pub fn auto_level_enemies(this: &PersonData, enemy_level: i32, index: usize){
    unsafe {
        let initial_level = INITIAL_LEVEL[index];
        let class_type = CLASS_LEVEL[index];
        if class_type == 0 { return; }
        let mut total_level = enemy_level;
        if is_boss(this){ total_level = 4 + enemy_level; }
        else if !Capability_is_zero(get_Grow(this, None), None) { total_level = 2 + enemy_level; }
        if total_level < initial_level.into() {
            total_level = initial_level.into();
        }
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
    get_initial_levels(); 
    if !GameVariableManager::get_bool( "G_Cleared_M004".into() ) { return; }
    if NG {  unsafe { if GameUserData::get_game_mode() == GameMode::Classic { GameUserData::set_game_mode(GameMode::Casual); } } }
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    let diff = GameUserData::get_difficulty(false);
    update_recommendedLevel();
    unsafe { 
        let mut player_average = GetAverageLevel(2, 14 - 2*diff, None) - 3;
        AVERAGE = player_average;
        if player_average < 1 { player_average = 1; }
        let new_enemy_Level = player_average + diff*2 - 1;
        println!("Player Army Average Level: {}", player_average);
        println!("NPC Level {}", new_enemy_Level);
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
                    new_person_level = (player_average as u8) - internal_level + 1;
                }
                else { 
                    person_total_level = person_internal_level  + initial_level; 
                    new_person_level = (player_average as u8) - person_internal_level + 1;
                }
                if new_person_level == 0 { new_person_level = 1; }
                if (i32::from(person_total_level) < player_average){ set_level(t_list[x], new_person_level.try_into().unwrap(), None); }
                else { set_level(t_list[x], initial_level.try_into().unwrap(), None);  }
            }
        }
        for x in 88..758 { auto_level_enemies(t_list[x], new_enemy_Level, x); }
        for x in 0..(FX_end-FX_start) { auto_level_enemies(t_list[FX_start + x], player_average, 800+x); }
        if player_average <= 20 {
            for x in 0..DLC.len() {
                let person = PersonData::get(DLC[x]);
                match person {
                    Some(p) => {
                        let job_dlc = GetJob(p, None);
                        let class_type = CLASS_LEVEL[970+x];
                        if job_is_low(job_dlc, None){ set_level(p, player_average.try_into().unwrap(), None); }
                        else { demote_person(p, player_average.try_into().unwrap(), CLASS_LEVEL[790+x]); }
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
                        let class_type = CLASS_LEVEL[790+x];
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

