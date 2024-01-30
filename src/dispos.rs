use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};
use engage::{force::*, gamevariable::*, gameuserdata::*, gamedata::unit::*};
use crate::engage_functions::*;
use crate::autolevel::*;
use crate::ng;
pub const EMBLEMS:  &[&str] = &[ "GID_M010_敵リン", "GID_M007_敵ルキナ", "GID_M014_敵ベレト", "GID_M024_敵マルス", "GID_M017_敵シグルド", "GID_M017_敵セリカ", "GID_M019_敵ミカヤ", "GID_M019_敵ロイ", "GID_M017_敵リーフ", "GID_E006_敵エーデルガルト", "GID_E006_敵クロム", "GID_E006_敵カミラ", "GID_E006_敵セネリオ", "GID_E006_敵ヴェロニカ", "GID_E006_敵ヘクトル", "GID_E006_敵チキ", "GID_M017_敵カムイ", "GID_M017_敵アイク", "GID_M017_敵エイリーク"];
pub const ENGAGE: &[&str] = &[ "AI_AT_EngageAttack", "AI_AT_EngageAttack", "AI_AT_EngageDance", "AI_AT_EngageAttack", "AI_AT_EngagePierce", "AI_AT_EngageAttack", "AI_AT_AttackToHeal", "AI_AT_EngageAttack", "AI_AT_EngageAttackNoGuard", "AI_AT_EngageClassPresident", "AI_AT_EngageAttack", "AI_AT_EngageCamilla", "AI_AT_EngageAttack", "AI_AT_EngageSummon", "AI_AT_EngageWait", "AI_AT_EngageBlessPerson", "AI_AT_EngageOverlap", "AI_AT_EngageWait", "AI_AT_EngageAttack"];

pub static mut EMBLEMS_USED: [bool; 20] = [false; 20];
use crate::autolevel::NG_KEY;

//Swap FX chapter copies with the actual unit
#[skyline::from_offset(0x01cfa570)]
pub fn disposdata_set_pid(this: &DisposData, value: &Il2CppString, method_info: OptionalMethod);
#[skyline::from_offset(0x01cfa5b0)]
pub fn disposdata_set_flag(this: &DisposData, value: &mut DisposData_FlagField, method_info: OptionalMethod);

pub fn replace_fx_data(){
    unsafe {
        //Assign Embelems and Revivals Stones to DisposData
        let t_list = DisposData::get_array_mut().expect("Me");
        let Ngroups = t_list.len();
        for x in 0..Ngroups {
            let Nunits = t_list[x].len();
            for y in 0..Nunits {
                let pid = disposdata_get_pid(t_list[x][y], None);
                if pid.is_some() {
                    let value = pid.unwrap();
                    if str_contains(value, "PID_E00"){
                        if str_contains(value, "_エル"){  disposdata_set_pid(t_list[x][y], "PID_エル".into(),  None);  }
                        else if str_contains(value, "エル_竜化"){ disposdata_set_pid(t_list[x][y], "PID_エル_竜化".into(),  None); }
                        else if str_contains(value, "_イル"){ 
                            if !GameVariableManager::get_bool("G_Cleared_E006"){ disposdata_set_pid(t_list[x][y], "PID_E004_イル".into(), None);  }
                            else { disposdata_set_pid(t_list[x][y], "PID_ラファール".into(),  None);   }
                        }
                        else if str_contains(value,"_セレスティア"){ disposdata_set_pid(t_list[x][y], "PID_セレスティア".into(),  None); }
                        else if str_contains(value,"_グレゴリー") { disposdata_set_pid(t_list[x][y], "PID_グレゴリー".into(),  None); }
                        else if str_contains(value, "_マデリーン"){ disposdata_set_pid(t_list[x][y], "PID_マデリーン".into(),  None); }
                    }
                    let sid = disposdata_get_sid(t_list[x][y], None);
                    if sid.is_some(){
                        let skill = sid.unwrap().get_string().unwrap(); 
                        if skill == "SID_虚無の呪い" { disposdata_set_sid(t_list[x][y], "SID_相手の命中１００".into(), None); } 
                    }
                }
                else {
                    continue;
                }
                let mut flags = disposdata_get_flag(t_list[x][y], None);
                if flags.value == 1807 {
                    if !str_contains(pid.unwrap(), "_イル") { 
                        flags.value = 911;
                        disposdata_set_flag(t_list[x][y], flags, None);
                    }
                }
            }
        }
    }
}

#[skyline::from_offset(0x01cfa5f0)]
pub fn disposdata_set_sid(this: &DisposData, value: &Il2CppString, method_info: OptionalMethod);

//to prevent the DLC characters from not being able to be deployed in FX 
#[skyline::hook(offset=0x01a0c6c0)]
pub fn unit_set_status(unit: &Unit, status: i64, method_info: OptionalMethod){
    if unit.m_Force.is_some() {
        if unit.m_Force.unwrap().m_Type == 1 && status == 67108864 { return;  }
    }
    // status that marks unit as defect and does not appear in the sortie
    if status == 1073741832 || status == 35184372088832 {  return;  }
    if status == 1073741824 { if unit.person.name.get_string().unwrap() == "MPID_Lueur" { return;  } }
    call_original!(unit, status, method_info);
}

pub fn all_revival_stones(){
    unsafe {
    let diff =  GameUserData::get_difficulty(false);
    for x in 0..20 { EMBLEMS_USED[x] = false; }
    let random = random_get_Game(None);
    let t_list = DisposData::get_array_mut().expect("Me");
    let Ngroups = t_list.len();
    let Nemblem = 1*<usize as TryInto<i32>>::try_into(EMBLEMS.len()).unwrap();
    let mut revivalStones_odds = 50;
    for x in 0..Ngroups {
        let Nunits = t_list[x].len();
        for y in 0..Nunits {
            let force = disposdata_get_force(t_list[x][y], None);
            if force != 1 { break; }
            let stone_rng = random_getMinMax(random, 0, 100, None);
            let flagValue = disposdata_get_flag(t_list[x][y], None).value;
            if stone_rng < 50|| (flagValue & 16) == 16 {
                let HP_count: u8 = disposdata_get_HPstockCount(t_list[x][y], None) + 1; 
                disposdata_set_HPstockcount(t_list[x][y], HP_count, None);
            }
            else if is_null_empty(disposdata_get_gid(t_list[x][y], None), None) && force == 1 {
                let rng: usize = random_getMinMax(random, 0, 3*Nemblem + 5, None).try_into().unwrap();
                if rng < Nemblem.try_into().unwrap(){
                    if can_use_emblem(t_list[x][y], rng) {
                        disposdata_set_gid(t_list[x][y], EMBLEMS[rng].into(), None);
                        disposdata_set_AI_attack_name(t_list[x][y],ENGAGE[rng].into(), None);
                        if diff == 2 && rng != 0 { disposdata_set_AI_attack_value(t_list[x][y],"2,2".into(),None); }
                        else { disposdata_set_AI_attack_value(t_list[x][y],"3,3".into(),None); }
                        if EMBLEMS[rng] == "GID_M017_敵カムイ" { disposdata_set_AI_attack_value(t_list[x][y],"255, 255, 3, 3".into(), None);  }
                        EMBLEMS_USED[rng] = true;
                        let god = GodData::get(EMBLEMS[rng].into()).unwrap().mid.get_string().unwrap();
                        println!("Emblem {}: {} is used ", rng, god);
                    }
                    else {
                        let HP_count: u8 = disposdata_get_HPstockCount(t_list[x][y], None) + 1; 
                        disposdata_set_HPstockcount(t_list[x][y], HP_count, None);
                    }
                }
                else {
                    let HP_count: u8 = disposdata_get_HPstockCount(t_list[x][y], None) + 1; 
                    disposdata_set_HPstockcount(t_list[x][y], HP_count, None);
                }
            }
        }
    }
}
}
#[skyline::hook(offset=0x029c4120)]
pub fn mapdispos_load(fileName: &Il2CppString, method_info: OptionalMethod){
    auto_level_persons();   // Autolevel Peeps here
    let cap = ng::calculate_player_cap();
    // load dispos here
    call_original!(fileName, method_info);
    replace_fx_data();
    // If FX then autolevel party 
    if str_contains(fileName, "E00")  {
        GameVariableManager::set_number("G_NG_CAP".into(), -1);
        unsafe {
            let instance = GameUserData::get_instance();
            let status = get_UserData_Status(instance, None);
            if status.value != 8192 {  // if not map replay 
                if str_contains(fileName, "E006"){ autolevel_party(10, 2, true); }
                else { autolevel_party(10, 3, true); }
                autolevel_DLC();
            }
        }
        return;
    }
    //Skirimish and //Divine Paralogue 
    if str_contains(fileName, "G00") && !str_contains(fileName, "E") {
        GameVariableManager::set_number("G_NG_CAP".into(), 20);
        return;
    }
    if str_contains(fileName, "E") {
        GameVariableManager::set_number("G_NG_CAP".into(), 0);
        return;
    }
    //revival stones for Chapter 22
    if GameVariableManager::get_bool(NG_KEY) && str_contains(fileName, "M022") {
        unsafe {
            let random = random_get_Game(None);
            let t_list = DisposData::get_array_mut().expect("Me");
            let Ngroups = t_list.len();
            for x in 0..Ngroups {
                let Nunits = t_list[x].len();
                if Nunits < 5 { continue; }
                for y in 0..Nunits {
                    let force = disposdata_get_force(t_list[x][y], None);
                    if force != 1 { break; }
                    if random_getMinMax(random, 0, 50, None) < 20 { disposdata_set_HPstockcount(t_list[x][y], 1, None); }
                }
            }
        }
        return;
    }
    if GameVariableManager::get_bool(NG_KEY) {
        unsafe {
            //Assign Embelems and Revivals Stones to DisposData
            let diff =  GameUserData::get_difficulty(false);
            for x in 0..20 { EMBLEMS_USED[x] = false; }
            let random = random_get_Game(None);
            let t_list = DisposData::get_array_mut().expect("Me");
            let Ngroups = t_list.len();
            let Nemblem = 1*<usize as TryInto<i32>>::try_into(EMBLEMS.len()).unwrap();
            let mut revivalStones_odds = 60;
            if GameVariableManager::get_bool( "G_Cleared_M007".into()) {revivalStones_odds = 50; }
            if str_contains(fileName, "S0") || GameVariableManager::get_bool( "G_Cleared_M017".into()) { revivalStones_odds = 35; }
            if GameVariableManager::get_bool( "G_Cleared_M020".into())  { revivalStones_odds = 25; }
            for x in 0..Ngroups {
                let Nunits = t_list[x].len();
                for y in 0..Nunits {
                    let force = disposdata_get_force(t_list[x][y], None);
                    if force != 1 { break; }
                    let stone_rng = random_getMinMax(random, 0, 100, None);
                    let flagValue = disposdata_get_flag(t_list[x][y], None).value;
                    if stone_rng < revivalStones_odds || (flagValue & 16) == 16 {
                        let HP_count: u8 = disposdata_get_HPstockCount(t_list[x][y], None) + 1;
                        disposdata_set_HPstockcount(t_list[x][y], HP_count, None);
                    }
                    else if is_null_empty(disposdata_get_gid(t_list[x][y], None), None) && force == 1 {
                        let rng: usize = random_getMinMax(random, 0, 3*Nemblem, None).try_into().unwrap();
                        if rng < Nemblem.try_into().unwrap(){
                            if can_use_emblem(t_list[x][y], rng) {
                                disposdata_set_gid(t_list[x][y], EMBLEMS[rng].into(), None);
                                disposdata_set_AI_attack_name(t_list[x][y],ENGAGE[rng].into(), None);
                                if diff == 2 && rng != 0 { disposdata_set_AI_attack_value(t_list[x][y],"2,2".into(),None); }
                                else { disposdata_set_AI_attack_value(t_list[x][y],"3,3".into(),None); }
                                if EMBLEMS[rng] == "GID_M017_敵カムイ" { disposdata_set_AI_attack_value(t_list[x][y],"255, 255, 3, 3".into(), None);  }
                                EMBLEMS_USED[rng] = true;
                                let god = GodData::get(EMBLEMS[rng].into()).unwrap().mid.get_string().unwrap();
                                println!("Emblem {}: {} is used ", rng, god);
                            }
                        }
                    }
                }
            }
        }
    }
}
#[unity::from_offset("App","DisposData", "GetPerson")]
pub fn DisposData_get_person(this: &DisposData, method_info: OptionalMethod) -> Option<&PersonData>;

#[unity::from_offset("App", "JobData", "get_Flag")]
pub fn Job_Get_Flag(this: &JobData, method_info: OptionalMethod) -> &'static mut UnitStatus;

#[skyline::from_offset(0x02053e20)]
pub fn get_job_style(this: &JobData, method_info: OptionalMethod) -> Option<&Il2CppString>;

pub fn can_use_emblem(data: &DisposData, emblem: usize) -> bool {
    unsafe {
        if GodData::get(EMBLEMS[emblem].into()).is_none() { return false; }
        if EMBLEMS_USED[emblem] { return false; }
        if DisposData_get_person(data, None).is_some() {
            let job = GetJob(DisposData_get_person(data, None).unwrap(), None);
            //Prevents Wyrms/Wolves from getting emblems
            if ( Job_Get_Flag(job, None).value == 0 && job.jid.get_string().unwrap() != "JID_蛮族" ) || Job_Get_Flag(job, None).value == 8 { return false; }
            let styleName = get_job_style(job, None);
            if styleName.is_some() {
                // Not Flying or Armored or wolf knight for Bow/Magic Emblems
                if styleName.unwrap().get_string().unwrap() == "飛行スタイル" || styleName.unwrap().get_string().unwrap() == "重装スタイル" || job.jid.get_string().unwrap() == "JID_ウルフナイト" {
                    match emblem {
                        0 | 1 | 5 | 6 | 11 | 12 | 13 => { return false; }
                        _ => { return true;}
                    }
                }
                else { return true; }
            }
            else { return false; }
        }
        else { return false; }   
    }
}