use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};
use engage::{force::*, gamevariable::*, gameuserdata::*, gamedata::unit::*};
use crate::engage_functions::*;
use crate::autolevel::*;
use skyline::patching::Patch;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};

pub const NG_KEY2: &str = "G_NG_OPTION";
pub struct NGMod;
impl ConfigBasicMenuItemSwitchMethods for NGMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        GameVariableManager::make_entry_norewind(NG_KEY2, 0);
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(NG_KEY2);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 3, 1);
        if toggle != result {
            GameVariableManager::set_number(NG_KEY2, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(NG_KEY2);
        if typeC == 0 {this.help_text = format!("No changes are made to units and inventory.").into(); }
        else if typeC == 1 {this.help_text = format!("Units will be reset to level 5 in their base class.").into(); }
        else if typeC == 2 {this.help_text = format!("Convoy, unit's inventory and levels will reset.").into(); }
        else if typeC == 3 {this.help_text = format!("Convoy, unit's inventory, levels, and skills will reset.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(NG_KEY2);
        if typeC == 0 {this.command_text = format!("No Reset").into(); }
        else if typeC == 1 {this.command_text = format!("Level Only").into(); }
        else if typeC == 2 {this.command_text = format!("Level and Inventory").into(); }
        else if typeC == 3 {this.command_text = format!("Full Reset").into(); }
    }
}
extern "C" fn ng() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<NGMod>("New Game+ Setting") }
pub fn ng_install(){ cobapi::install_game_setting(ng); }

pub fn find_personIndex(pid: &Il2CppString) -> usize {
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list.items;
    unsafe {
        for x in 0..760 {
            if string_contains(t_list[x].pid, pid, None) {
                return x;
            }
        }
    }
    return 0;
}

pub fn unit_cap_total(this: &Unit) -> i32 {
    let mut total = 0;
    unsafe {
        for x in 1..9 { total = total + unit_get_capability(this, x, false, None); }
    }
    total
}
#[skyline::from_offset(0x01a3f480)]
pub fn unit_add_itemList(this: &Unit, iid: u64, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "Unit", "ItemPutOffAll")]
pub fn unit_item_put_off_all(this: &Unit, method_info: OptionalMethod);

#[unity::from_offset("App", "JobData", "get_UniqueItems")]
pub fn jobdata_get_uniqueItems(this: &JobData, method_info: OptionalMethod) -> u64;

#[skyline::from_offset(0x01a0c990)]
pub fn unit_add_item_iid(this: &Unit, iid: &Il2CppString, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App","SkillArray", "Clear")]
pub fn skillarray_clear(this: &SkillArray, method_info: OptionalMethod);

#[skyline::from_offset(0x02334600)]
pub fn TryGetGod(gid: &GodData, includedReserved: bool,  method_info: OptionalMethod) -> Option<&GodUnit>;
#[skyline::from_offset(0x02340a30)]
pub fn set_god_level(this: &GodUnit, unit: &Unit, level: i32, method_info: OptionalMethod);

pub fn set_bonds_to_level(unit: &Unit, level: i32){
    unsafe {
    let triabolical3 = &GodData::get_list().expect("triabolical2 is 'None'").list.items;
    for i in 0..100 {
        let result = TryGetGod(&triabolical3[i], true, None);
        if result.is_some() {
            if result.unwrap().m_Data.gid.get_string().unwrap() == "GID_リュール" {
                continue;
            }
            else { set_god_level(result.unwrap(), unit, level, None); }
        }
    }
    }
}

pub fn reset_units(level: i32){
    unsafe {
        let rng = random_get_Game(None);
        let typeC =  GameVariableManager::get_number(NG_KEY2);
        if typeC == 0 {
            autolevel_party(10, 4, false);
            return;
        }
        let triabolical2 = JobData::get_list().expect("triabolical2 is 'None'");
        let t_list2 = &triabolical2.list.items;
        let mut n_master_seals: i32 = 0;
        for force in 0..7 {
            let benchForce = Force_Get(force, None);
            let mut force_iter = Force::iter(benchForce);
            while let Some(unit) = force_iter.next() {
                let person = &unit.person;
                let job = GetJob(person, None);
                let mut new_job = job;
                unit.class_change(t_list2[1]); // class change into this class to remove job skill
                let T_level: i32 = (unit.m_Level as i32) + (unit.m_InternalLevel as i32);
                if typeC == 2 { unit_item_put_off_all(unit, None); }
                let apt = person_get_apt(person, None);
                let apt_value = apt.value;
                apt.value = 1023;
                if person.pid.get_string().unwrap() == "PID_ヴァンドレ" {
                    if typeC == 3 { 
                        skillarray_clear(unit.m_EquipSkillPool, None);
                        skillarray_clear(unit.m_EquipSkill,None);
                    } 
                    unit_CreateImpl1(unit, person, job, 1, rng, None); 
                    unit_set_level(unit, 1, None);
                    unit_set_internal_level(unit, 0, None);
                }
                else {
                    if !job_is_low(job, None){ 
                        n_master_seals += 1;
                        let low_job = job_GetLowJobs(job, None);
                        if low_job.len() >= 3 {
                            if person.pid.get_string().unwrap() == "PID_マデリーン" { 
                                let selection: usize = (CLASS_LEVEL[794]-1).into();
                                new_job = low_job.items[selection];
                            }
                            else if person.pid.get_string().unwrap() == "PID_ロサード" {   new_job = low_job.items[2]; }
                            else {  new_job = low_job.items[0]; }
                        }
                        else if low_job.len() == 0 {  new_job = job; }
                        else {
                            new_job = low_job.items[0];
                        }
                        unit_CreateImpl1(unit, person, new_job, 1, rng, None); 
                        if typeC == 3 {
                            skillarray_clear(unit.m_EquipSkillPool, None);
                            skillarray_clear(unit.m_EquipSkill,None);
                        }
                    }
                    else {
                        unit_CreateImpl1(unit, person, new_job, level-1, rng, None); 
                        if typeC == 3 {
                            skillarray_clear(unit.m_EquipSkillPool, None);
                            skillarray_clear(unit.m_EquipSkill,None);
                        }
                    }
                    unit_set_level(unit, level, None);
                    unit_set_internal_level(unit, 0, None);
                    let mut baseCap = unit_cap_total(unit);
                    let previousHP = unit_get_Hp(unit, None);
                    let HP_capability = unit_get_capability(unit, 0, false, None);
                    while 70 < baseCap {
                        Unit_LevelDown(unit, None);
                        unit_set_level(unit, level, None);
                        baseCap = unit_cap_total(unit);
                    }
                    unit_set_level(unit, level, None);
                    let new_HP_capability = unit_get_capability(unit, 0, true, None);
                    let new_HP = previousHP + new_HP_capability - previousHP;
                    unit_set_Hp(unit, new_HP, None);
                }
                if typeC == 3 { set_bonds_to_level(unit, 5); }
                if T_level*100 < 3000 { unit_set_SP(unit, 3000, None);}
                else { unit_set_SP(unit, T_level*100, None); }
                unit_set_exp(unit, 0, None);
                apt.value = apt_value;
                person_set_apt(person, apt, None);
                if typeC >= 2 && person.pid.get_string().unwrap() == "PID_エル" && GameVariableManager::get_number("G_拠点_裏武器イベント") > 1 {
                    unit_add_item_iid(unit, "IID_トライゾン".into(), None);
                }
                else if typeC >= 2 && person.pid.get_string().unwrap() == "PID_ラファール" && GameVariableManager::get_number("G_拠点_裏武器イベント") > 1 {
                    unit_add_item_iid(unit, "IID_ルヴァンシュ".into(), None);
                }
                else if typeC >= 2 && person.pid.get_string().unwrap() == "PID_ヴェイル" {
                    unit_add_itemList(unit, jobdata_get_uniqueItems(new_job, None), None); 
                    unit_add_item_iid(unit, "IID_ミセリコルデ".into(), None);
                }
                else if typeC >= 2 && job_is_low(new_job, None) && job_has_high_job(new_job, None) {
                    unit_item_put_off_all(unit, None);
                    unit_add_itemList(unit, jobdata_get_uniqueItems(new_job, None), None);
                }
                else if typeC >= 2 { unit_add_itemList(unit, jobdata_get_uniqueItems(new_job, None), None); }
            }
        }
        force_transfer(Force_Get(4, None), 3, true, None);
        let instance = GameUserData::get_instance();
        if typeC >= 2 {
            transporter_reset(None);
            set_iron(instance, 200, None);
            set_steel(instance, 20, None);
            set_silver(instance, 20, None);
            set_gold(instance, 40000, None);
            set_PieceOfBond(instance, 10000, None);
            // Seals
            GameVariableManager::set_number("G_所持_IID_マスタープルフ".into(), n_master_seals);
            GameVariableManager::set_number("G_所持_IID_チェンジプルフ".into(), 3); //second
            if GameVariableManager::get_bool( "G_Cleared_E006".into()){
                GameVariableManager::set_number("G_所持_IID_エンチャント専用プルフ".into(), 1); //DLC seals
                GameVariableManager::set_number("G_所持_IID_マージカノン専用プルフ".into(), 1);
            }

        }
        //Adding 
        else {
            GameVariableManager::set_number("G_所持_IID_マスタープルフ".into(), n_master_seals + GameVariableManager::get_number("G_所持_IID_マスタープルフ".into()));
            GameVariableManager::set_number("G_所持_IID_チェンジプルフ".into(), 3 + GameVariableManager::get_number("G_所持_IID_チェンジプルフ".into()));
            if GameVariableManager::get_bool( "G_Cleared_E006".into()){
                GameVariableManager::set_number("G_所持_IID_エンチャント専用プルフ".into(), 1 + GameVariableManager::get_number("G_所持_IID_エンチャント専用プルフ".into()));
                GameVariableManager::set_number("G_所持_IID_マージカノン専用プルフ".into(), 1 + GameVariableManager::get_number("G_所持_IID_エンチャント専用プルフ".into()));
            }
        }
    }
}
#[skyline::from_offset(0x0250e450)]
pub fn set_gold(this: &GameUserData, value : i32, method_info: OptionalMethod);

//Reset World Map and autolevels player units for NG+ when Chapter 26 is completed
pub fn resetGmap(){
    GameVariableManager::make_entry_norewind(NG_KEY, 0);
    if GameVariableManager::get_bool("G_Cleared_M026".into()) {
        GameVariableManager::set_bool( "G_Cleared_M005".into(), false);
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
        GameVariableManager::set_bool( "G_Cleared_S002".into(), false);
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
        GameVariableManager::set_number( "G_GmapSpot_M006".into(), 1);
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

        GameVariableManager::set_number( "G_GmapSpot_S002".into(), 1);
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
        reset_units(5);
    }
}

// Hooking to refresh gmap for NG+
#[skyline::hook(offset=0x02b3a3f0)]
pub fn gmap_load(this: &u64, method_info: OptionalMethod){
    call_original!(this, method_info);
    resetGmap();
}