use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};
use engage::{force::*, gamevariable::*, gameuserdata::*, gamedata::unit::*};
use crate::engage_functions::*;
pub const EMBLEMS:  &[&str] = &[ "GID_M010_敵リン", "GID_M007_敵ルキナ", "GID_M014_敵ベレト", "GID_M024_敵マルス", "GID_M017_敵シグルド", "GID_M017_敵セリカ", "GID_M017_敵ミカヤ", "GID_M019_敵ロイ", "GID_M017_敵リーフ", "GID_E006_敵エーデルガルト", "GID_E006_敵クロム", "GID_E006_敵カミラ", "GID_E006_敵セネリオ", "GID_E006_敵ヴェロニカ", "GID_E006_敵ヘクトル", "GID_E006_敵チキ"];
pub const ENGAGE: &[&str] = &[ "AI_AT_EngageAttack", "AI_AT_EngageAttack", "AI_AT_EngageDance", "AI_AT_EngageAttack", "AI_AT_EngagePierce", "AI_AT_EngageAttack", "AI_AC_AttackRange", "AI_AT_EngageAttack", "AI_AT_EngageAttackNoGuard", "AI_AT_EngageAttack", "AI_AT_EngageAttack", "AI_AT_EngageCamilla", "AI_AT_EngageAttack", "AI_AT_Attack", "AI_AT_Attack", "AI_AT_EngageBlessPerson"];
pub static mut EMBLEMS_USED: [bool; 20] = [false; 20];
use crate::autolevel::NG_KEY;

#[unity::class("App","DisposData_FlagField")]
pub struct DisposData_FlagField {
    pub value : i32,
}

#[skyline::from_offset(0x01cfa830)]
pub fn disposdata_set_gid(this: &DisposData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x01cfa5a0)]
pub fn disposdata_get_flag(this: &DisposData, method_info: OptionalMethod) -> &DisposData_FlagField;

#[skyline::from_offset(0x01cfa820)]
pub fn disposdata_get_gid(this: &DisposData, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x01cfab40)]
pub fn disposdata_get_force(this: &DisposData, method_info: OptionalMethod) -> i8;

#[skyline::from_offset(0x01cfa9b0)]
pub fn disposdata_set_AI_attack_name(this: &DisposData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x01cfa840)]
pub fn disposdata_get_HPstockCount(this: &DisposData, method_info: OptionalMethod) -> u8;

#[skyline::from_offset(0x01cfa850)]
pub fn disposdata_set_HPstockcount(this: &DisposData, value: u8, method_info: OptionalMethod);

#[skyline::hook(offset=0x029c4120)]
pub fn mapdispos_load(fileName: &Il2CppString, method_info: OptionalMethod){
    call_original!(fileName, method_info);
    let NG = GameVariableManager::get_bool(NG_KEY);
    if NG {
        unsafe {
            for x in 0..20 { EMBLEMS_USED[x] = false; }
            let random = random_get_Game(None);
            let t_list = DisposData::get_array_mut().expect("Me");
            let Ngroups = t_list.len();
            let emblemOdds = 3*<usize as TryInto<i32>>::try_into(EMBLEMS.len()).unwrap();
            for x in 0..Ngroups {
                let Nunits = t_list[x].len();
                for y in 0..Nunits {
                    let force = disposdata_get_force(t_list[x][y], None);
                    if force != 1 {
                        println!("Group {}, is force {}", x, force);
                        break;
                    }
                    let stone_rng = random_getMinMax(random, 0, 100, None);
                    let flagValue = disposdata_get_flag(t_list[x][y], None).value;
                    if stone_rng < 50 || (flagValue & 16) == 16 {
                        let HP_count: u8 = disposdata_get_HPstockCount(t_list[x][y], None) + 1; 
                        disposdata_set_HPstockcount(t_list[x][y], HP_count, None);
                    }
                    else if is_null_empty(disposdata_get_gid(t_list[x][y], None), None) && force == 1 {
                        let rng: usize = random_getMinMax(random, 0, emblemOdds, None).try_into().unwrap();
                        if rng < EMBLEMS.len().try_into().unwrap(){
                            if !EMBLEMS_USED[rng] {
                                disposdata_set_gid(t_list[x][y], EMBLEMS[rng].into(), None);
                                disposdata_set_AI_attack_name(t_list[x][y],ENGAGE[rng].into(), None);
                                EMBLEMS_USED[rng] = true;
                            }
                        }
                    }
                }
            }
        }
    }
}
