use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};
use engage::{force::*, gamevariable::*, gameuserdata::*, gamedata::unit::*};
use crate::engage_functions::*;
use crate::autolevel::*;
pub const EMBLEMS:  &[&str] = &[ "GID_M010_敵リン", "GID_M007_敵ルキナ", "GID_M014_敵ベレト", "GID_M024_敵マルス", "GID_M017_敵シグルド", "GID_M017_敵セリカ", "GID_M019_敵ミカヤ", "GID_M019_敵ロイ", "GID_M017_敵リーフ", "GID_E006_敵エーデルガルト", "GID_E006_敵クロム", "GID_E006_敵カミラ", "GID_E006_敵セネリオ", "GID_E006_敵ヴェロニカ", "GID_E006_敵ヘクトル", "GID_E006_敵チキ", "GID_M017_敵カムイ", "GID_M017_敵アイク", "GID_M017_敵エイリーク"];
pub const ENGAGE: &[&str] = &[ "AI_AT_EngageAttack", "AI_AT_EngageAttack", "AI_AT_EngageDance", "AI_AT_EngageAttack", "AI_AT_EngagePierce", "AI_AT_EngageAttack", "AI_AT_AttackToHeal", "AI_AT_EngageAttack", "AI_AT_EngageAttackNoGuard", "AI_AT_EngageClassPresident", "AI_AT_EngageAttack", "AI_AT_EngageCamilla", "AI_AT_EngageAttack", "AI_AT_EngageSummon", "AI_AT_EngageWait", "AI_AT_EngageBlessPerson", "AI_AT_EngageOverlap", "AI_AT_EngageWait", "AI_AT_EngageAttack"];

pub static mut EMBLEMS_USED: [bool; 20] = [false; 20];
use crate::autolevel::NG_KEY;

//Swap FX chapter copies with the actual unit
#[skyline::hook(offset=0x01cfa570)]
pub fn disposdata_set_pid(this: &DisposData, value: &Il2CppString, method_info: OptionalMethod){
    if str_contains(value, "PID_E00"){
        if str_contains(value, "_エル"){  call_original!(this, "PID_エル".into(), method_info);  }
        else if str_contains(value, "エル_竜化"){ call_original!(this, "PID_エル_竜化".into(), method_info); }
        else if str_contains(value, "_イル"){ 
            if !GameVariableManager::get_bool("G_Cleared_E006"){ call_original!(this, "PID_E004_イル".into(), method_info);  }
            else { call_original!(this, "PID_ラファール".into(), method_info);   }
        }
        else if str_contains(value,"_セレスティア"){ call_original!(this, "PID_セレスティア".into(), method_info); }
        else if str_contains(value,"_グレゴリー") { call_original!(this, "PID_グレゴリー".into(), method_info); }
        else if str_contains(value, "_マデリーン"){ call_original!(this, "PID_マデリーン".into(), method_info); }
        else { call_original!(this, value, method_info); }
    }
    else { call_original!(this, value, method_info); }
}
#[skyline::hook(offset=0x01cfa5b0)]
pub fn disposdata_set_flag(this: &DisposData, value: &mut DisposData_FlagField, method_info: OptionalMethod){
    if value.value == 1807 {
        unsafe { 
            if !str_contains(disposdata_get_pid(this, None), "_イル") { 
                value.value = 911;
                call_original!(this, value, method_info);
                return;
            }
            else {  call_original!(this, value, method_info); }
        }
    }
    else { call_original!(this, value, method_info); }
}
//to prevent the DLC characters from not being able to be deployed in FX 
#[skyline::hook(offset=0x01a0c6c0)]
pub fn unit_set_status(unit: &Unit, status: i64, method_info: OptionalMethod){
    // status that marks unit as defect and does not appear in the sortie
    if status == 1073741832  { return;  }
    if status == 1073741824 {
        if unit.person.name.get_string().unwrap() == "MPID_Lueur" { return;  }
    }
    call_original!(unit, status, method_info);

}
#[skyline::hook(offset=0x029c4120)]
pub fn mapdispos_load(fileName: &Il2CppString, method_info: OptionalMethod){
    auto_level_persons();   // Autolevel Peeps here
    // If FX then autolevel party 
    if str_contains(fileName, "E00"){
        unsafe {
            let instance = GameUserData::get_instance();
            let status = get_UserData_Status(instance, None);
            if status.value != 8192 {  // if not map replay 
                if str_contains(fileName, "E006"){ autolevel_party(10, 2, true); }
                else { autolevel_party(10, 3, true); }
                autolevel_DLC();
            }
        }
    }
    if GameVariableManager::get_bool(NG_KEY) {
        unsafe {
            call_original!(fileName, method_info);
            let diff =  GameUserData::get_difficulty(false);
            for x in 0..20 { EMBLEMS_USED[x] = false; }
            let random = random_get_Game(None);
            let t_list = DisposData::get_array_mut().expect("Me");
            let Ngroups = t_list.len();
            let Nemblem = 1*<usize as TryInto<i32>>::try_into(EMBLEMS.len()).unwrap();
            for x in 0..Ngroups {
                let Nunits = t_list[x].len();
                for y in 0..Nunits {
                    let force = disposdata_get_force(t_list[x][y], None);
                    if force != 1 { break; }
                    let stone_rng = random_getMinMax(random, 0, 100, None);
                    let flagValue = disposdata_get_flag(t_list[x][y], None).value;
                    if stone_rng < 50 || (flagValue & 16) == 16 {
                        let HP_count: u8 = disposdata_get_HPstockCount(t_list[x][y], None) + 1; 
                        disposdata_set_HPstockcount(t_list[x][y], HP_count, None);
                    }
                    else if is_null_empty(disposdata_get_gid(t_list[x][y], None), None) && force == 1 {
                        let rng: usize = random_getMinMax(random, 0, 3*Nemblem+1, None).try_into().unwrap();
                        if rng < Nemblem.try_into().unwrap(){
                            if !EMBLEMS_USED[rng] && GodData::get(EMBLEMS[rng].into()).is_some() {
                                disposdata_set_gid(t_list[x][y], EMBLEMS[rng].into(), None);
                                disposdata_set_AI_attack_name(t_list[x][y],ENGAGE[rng].into(), None);
                                if diff == 2 { disposdata_set_AI_attack_value(t_list[x][y],"2,2".into(),None); }
                                else { disposdata_set_AI_attack_value(t_list[x][y],"3,3".into(),None); }
                                if EMBLEMS[rng] == "GID_M017_敵カムイ" { disposdata_set_AI_attack_value(t_list[x][y],"255, 255, 3, 3".into(), None);  }
                                EMBLEMS_USED[rng] = true;
                            }
                        }
                    }
                }
            }
        }
    }
    else { call_original!(fileName, method_info);  }
}