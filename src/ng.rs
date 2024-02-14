use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*};
use engage::{force::*, gamevariable::*, gameuserdata::*, gamedata::unit::*};
use crate::engage_functions::*;
use crate::autolevel::*;
use crate::misc::*;
use crate::autolevel;
use skyline::patching::Patch;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use unity::system::List;
use unity::il2cpp::object::Array;
pub static mut AVERAGE_CAP: i32 = 0;
pub const NG_KEY2: &str = "G_NG_OPTION";
pub const RSH_KEY: &str = "G_RSH";

pub struct NGMod;
impl ConfigBasicMenuItemSwitchMethods for NGMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ 
        GameVariableManager::make_entry(NG_KEY2, 0);
     }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(NG_KEY2);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 4, 1);
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
        if typeC == 0 {this.help_text = "No changes are made to units and inventory.".into(); }
        else if typeC == 1 {this.help_text = "Units will be reset to level 5 in their base class.".into(); }
        else if typeC == 2 {this.help_text = "Convoy, unit's inventory, and levels will reset.".into(); }
        else if typeC == 3 {this.help_text = "Convoy, unit's inventory, levels, bonds, and skills will reset.".into(); }
        else if typeC == 4 {this.help_text = "Game will not reset after completing Chapter 26".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(NG_KEY2);
        if typeC == 0 {this.command_text = "No Reset".into(); }
        else if typeC == 1 {this.command_text = "Level Only".into(); }
        else if typeC == 2 {this.command_text = "Level and Inventory".into(); }
        else if typeC == 3 {this.command_text = "Full Reset".into(); }
        else if typeC == 4 {this.command_text = Off_str(); }
    }
}
extern "C" fn ng() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_switch::<NGMod>("New Game+ Setting") 
}
pub fn ng_install(){ cobapi::install_game_setting(ng); }

pub struct RSMod;
impl ConfigBasicMenuItemSwitchMethods for RSMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ 
       GameVariableManager::make_entry(RSH_KEY, 0);
     }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle = GameVariableManager::get_number(RSH_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 1, 1);
        if toggle != result {
            GameVariableManager::set_number(RSH_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_number(RSH_KEY) == 1  {
            this.help_text = "All enemies will have a revivial stone or a dark emblem.".into();
        }
        else {
            this.help_text = "Default setting for enemies.".into();
       }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if GameVariableManager::get_number(RSH_KEY) == 1 {
            this.command_text = On_str().into();
        }
        else {
            this.command_text =  Off_str();
        }
    }
}
extern "C" fn rsh() -> &'static mut ConfigBasicMenuItem {
     ConfigBasicMenuItem::new_switch::<RSMod>("Revival Stone Hell") 
}
pub fn rsh_install(){ cobapi::install_game_setting(rsh); }


pub fn find_personIndex(pid: &Il2CppString) -> usize {
    let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
    let t_list = &triabolical.list;
    unsafe {
        for x in 0..760 {
            if string_contains(t_list[x].pid, pid, None) { return x;  }
        }
    }
    return 0;
}

pub fn unit_cap_total(this: &Unit, with_HP: bool) -> i32 {
    let mut total = 0;
    unsafe {
        for x in 1..8 { total = total + unit_get_capability(this, x, false, None); }
        if with_HP {
            total += unit_get_capability(this, 0, false, None);
            total += 2*unit_get_capability(this, 8, false, None);
            total += 2*unit_get_capability(this, 9, false, None);
            if unit_get_capability(this, 10, false, None) < 4 { total += unit_get_capability(this, 6, false, None); }
            else { total += (unit_get_capability(this, 10, false, None) - 4) * 10; }
        }
    }
    total
}
// this version checks for negative stats first
pub fn unit_cap_total_mut(this: &mut Unit, with_HP: bool) -> i32 {
    let mut total = 0;
    unsafe {
        //checking for negatives stat and corrects it
        for x in 0..11 {
            if unit_get_capability(this, x, false, None) < 0 {
                this.m_BaseCapability.capability.m_items[x as usize] = 127;
                if x >= 8 { this.m_BaseCapability.capability.m_items[x as usize] = 99; }
            }
        }
        for x in 1..8 { total = total + unit_get_capability(this, x, false, None); }
        if with_HP {
            total += unit_get_capability(this, 0, false, None);
            total += 2*unit_get_capability(this, 8, false, None);
            total += 2*unit_get_capability(this, 9, false, None);
            if unit_get_capability(this, 10, false, None) < 4 { total += unit_get_capability(this, 6, false, None); }
            else { total += (unit_get_capability(this, 10, false, None) - 4) * 10; }
        }
    }
    total
}

#[skyline::from_offset(0x0250e450)]
pub fn set_gold(this: &GameUserData, value : i32, method_info: OptionalMethod);

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

#[unity::from_offset("App", "ShopData", "Regist")]
pub fn shop_reset(method_info: OptionalMethod);

#[unity::from_offset("App", "WeaponShopData", "Regist")]
pub fn wshop_reset(method_info: OptionalMethod);

#[unity::from_offset("App", "ItemShopData", "Regist")]
pub fn ishop_reset(method_info: OptionalMethod);

#[skyline::from_offset(0x02340a30)]
pub fn set_god_level(this: &GodUnit, unit: &Unit, level: i32, method_info: OptionalMethod);

pub fn set_bonds_to_level(unit: &Unit, level: i32){
    unsafe {
        let triabolical3 = &GodData::get_list().expect("triabolical2 is 'None'").list.items;
        for i in 0..90 {
            let result = TryGetGod(&triabolical3[i], true, None);
            if result.is_some() {
                if result.unwrap().m_Data.gid.get_string().unwrap() == "GID_リュール" { continue; }
                else { set_god_level(result.unwrap(), unit, level, None); }
            }
        }
    }
}

pub fn reset_bond_unlock(){
    GameVariableManager::set_bool("G_マルスレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_シグルドレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_セリカレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_ミカヤレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_ロイレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_リーフレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_ルキナレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_リンレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_ベレトレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_カムイレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_エイリークレベルキャップ解放".into(), false);
    GameVariableManager::set_bool("G_アイクレベルキャップ解放".into(), false);
}

pub fn get_weapon_apt(this: &PersonData) -> usize {
    unsafe {
        let apt = person_get_apt(this, None);
        if apt.value & 2 == 2 { return 0;  }
        else if apt.value & 4 == 4 { return 1; }
        else if apt.value & 8 == 8 { return 2; }

        let apt2 = person_get_sub_apt(this, None);
        if apt2.value & 2 == 2 { return 0; }
        else if apt2.value & 4 == 4 { return 1; }
        else if apt2.value & 8 == 8 { return 2; }
    }
    return 0;
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
            for unit in force_iter {
                let person = &unit.person;
                let job = GetJob(person, None);
                let mut new_job = job;
                unit.class_change(t_list2[1]); // class change into this class to remove job skill
                let T_level: i32 = (unit.m_Level as i32) + (unit.m_InternalLevel as i32);
                if typeC == 2 { unit_item_put_off_all(unit, None); }
                let apt = person_get_sub_apt(person, None);
                let apt_value = apt.value;
                if person.pid.get_string().unwrap() == "PID_ヴァンドレ" {
                    if typeC == 3 { 
                        skillarray_clear(unit.m_EquipSkillPool, None);
                        skillarray_clear(unit.m_EquipSkill,None);
                    } 
                    apt.value = 1023;
                    unit_CreateImpl1(unit, person, job, 1, rng, None); 
                    unit_set_level(unit, 1, None);
                    unit_set_internal_level(unit, level-1, None);
                }
                else {
                    if !job_is_low(job, None){ 
                        n_master_seals += 1;
                        let low_job = job_GetLowJobs(job, None);
                        if low_job.len() >= 3 {
                            let selection: usize = get_weapon_apt(person);
                            new_job = low_job.items[selection];
                        }
                        else if low_job.len() == 0 {  new_job = job; }
                        else { new_job = low_job.items[0]; }
                        apt.value = 1023;
                        unit_CreateImpl1(unit, person, new_job, 1, rng, None); 
                    }
                    else {
                        apt.value = 1023;
                        unit_CreateImpl1(unit, person, new_job, level, rng, None); 
                    }
                    if typeC == 3 {
                        skillarray_clear(unit.m_EquipSkillPool, None);
                        skillarray_clear(unit.m_EquipSkill,None);
                    }
                    unit_set_level(unit, level, None);
                    unit_set_internal_level(unit, 0, None);
                    let mut baseCap = unit_cap_total(unit, false);
                    let previousHP = unit_get_Hp(unit, None);
                    let HP_capability = unit_get_capability(unit, 0, false, None);
                    while 60 < baseCap {
                        Unit_LevelDown(unit, None);
                        unit_set_level(unit, level, None);
                        baseCap = unit_cap_total(unit, false);
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
                person_set_sub_apt(person, apt, None);
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
            if typeC == 3 { reset_bond_unlock(); }
            transporter_reset(None);
            set_iron(instance, 200, None);
            set_steel(instance, 20, None);
            set_silver(instance, 20, None);
            set_gold(instance, 40000, None);
            set_PieceOfBond(instance, 10000, None);
            // Seals
            GameVariableManager::set_number("G_所持_IID_マスタープルフ".into(), 5+n_master_seals);
            GameVariableManager::set_number("G_所持_IID_チェンジプルフ".into(), 10); //second
            if GameVariableManager::get_bool( "G_Cleared_E006".into()){
                GameVariableManager::set_number("G_所持_IID_エンチャント専用プルフ".into(), 1); //DLC seals
                GameVariableManager::set_number("G_所持_IID_マージカノン専用プルフ".into(), 1);
            }
            reset_shop_added_stock();
        }
        //Adding 
        else {
            GameVariableManager::set_number("G_所持_IID_マスタープルフ".into(), 5+n_master_seals + GameVariableManager::get_number("G_所持_IID_マスタープルフ".into()));
            GameVariableManager::set_number("G_所持_IID_チェンジプルフ".into(), 10 + GameVariableManager::get_number("G_所持_IID_チェンジプルフ".into()));
            if GameVariableManager::get_bool( "G_Cleared_E006".into()){
                GameVariableManager::set_number("G_所持_IID_エンチャント専用プルフ".into(), 1 + GameVariableManager::get_number("G_所持_IID_エンチャント専用プルフ".into()));
                GameVariableManager::set_number("G_所持_IID_マージカノン専用プルフ".into(), 1 + GameVariableManager::get_number("G_所持_IID_エンチャント専用プルフ".into()));
            }
        }
    }
}
#[unity::from_offset("App", "GameVariable", "FindStartsWith")]
pub fn GameVariable_FindStartWith(this: &GameVariable, name: &Il2CppString, method_info: OptionalMethod) -> &'static List<Il2CppString>;

pub fn reset_shop_added_stock(){
    unsafe {
        shop_reset(None);
        wshop_reset(None);
        ishop_reset(None);
        let variable = GameUserData::get_variable();
        let list = GameVariable_FindStartWith(variable, "G_在庫追加_道具屋_".into(), None);
        if list.len() > 3 {
            for x in 3..list.len() { set_bool(variable, list[x], false, None); }
        }
        let list2 = GameVariable_FindStartWith(variable, "G_在庫_".into(), None);
        for x in 0..list2.len() {
            if get_number(variable, list2[x], None) == 0 {
                set_number(variable, list2[x], 1, None);
            }
        }
    }
}

//Reset World Map and autolevels player units for NG+ when Chapter 26 is completed
pub fn resetGmap(){
    GameVariableManager::make_entry_norewind(NG_KEY, 0);
    if GameVariableManager::get_bool("G_Cleared_M026".into()) &&  GameVariableManager::get_number(NG_KEY2) != 4 {
        println!("Clearing maps");
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

        GameVariableManager::set_number( "G_GmapSpot_S002".into(), 3);
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
//Calculating top average rating 
pub fn calculate_player_cap() -> i32 {
    let mut max_cap: [i32; 10] = [0; 10];
    let mut unit_name: [&Il2CppString; 10] = [" N/A".into(); 10];
    GameVariableManager::make_entry_norewind("G_NG_CAP", 0);
    unsafe {
        for force in 0..max_cap.len() {
            for ff in 0..6 {
                if ff == 1 && ff == 2 { continue; }
                let benchForce = Force_Get(ff, None);
                let mut force_iter = Force::iter(benchForce);
                let i: usize = force.into();
                for unit in force_iter {
                    if unit.person.name.get_string().unwrap() == "MPID_Vandre" { continue; }
                    if unit.person.name.get_string().unwrap() == "MPID_Mauve" { continue; }
                    let cur = unit_cap_total(unit, true);
                    if force == 0 {
                        if max_cap[i] < cur {
                            max_cap[i] = cur;
                            unit_name[i] = unit.person.name; 
                        }
                    }
                    else {
                        if max_cap[i] < cur && cur < max_cap[i-1] {
                            max_cap[i] = cur;
                            unit_name[i] = unit.person.name; 
                        }
                    }
                }
            }
        }   
    }
    let mut average: i32 = 0;
    let diff = GameUserData::get_difficulty(false);
    let count_average: usize = max_cap.len() - (2*diff as usize);
    for i in 0..count_average {
        average += max_cap[i] / (count_average as i32 );
        println!("Rank {}: {}/{} with rating {}", i+1, get_str(unit_name[i]), unit_name[i].get_string().unwrap(), max_cap[i]);
    }
    println!("{} unit Average is {}", count_average, average);
    GameVariableManager::set_number("G_NG_CAP".into(), average);
    average
}
// Hooking to refresh gmap for NG+, update recommended levels
#[skyline::hook(offset=0x02b3a3f0)]
pub fn gmap_load(this: &u64, method_info: OptionalMethod) {
    update_recommendedLevel();
    call_original!(this, method_info);
    //Engraving bypass and recall bypass
    if GameVariableManager::get_bool("G_NG") { 
        Patch::in_text(0x0295d5c8).bytes(&[0x00, 0x00, 0x80, 0xD2]).unwrap();
        Patch::in_text(0x02b415f8).bytes(&[0x01, 0x01, 0x80, 0x52]);
        Patch::in_text(0x01be1388).bytes(&[0x81, 0x00, 0x80, 0x52]);    // 4 stats
    }
    else { 
        Patch::in_text(0x0295d5c8).bytes(&[0xB2, 0xE1, 0xD8, 0x97]).unwrap(); 
        Patch::in_text(0x02b415f8).bytes(&[0x01, 0x04, 0x80, 0x52]);
        Patch::in_text(0x01be1388).bytes(&[0x61, 0x00, 0x80, 0x52]);    // 3 stats
    }
    resetGmap();
}


pub fn promote_unit(this: &Unit, level: i32){
    unsafe {
        let jobmaxLevel = this.m_Job.MaxLevel as i32;
        if jobmaxLevel < level {
            if job_is_low(this.m_Job, None) {
                let high_job = job_get_high_job1(this.m_Job, None);
                if !is_null_empty(high_job, None){
                    let hjob = JobData::get(&high_job.get_string().unwrap());
                    if hjob.is_some(){ 
                        this.class_change(hjob.unwrap());
                        println!("{} {} was promoted to {}", get_str(this.m_Job.name),  get_str(this.person.name), get_str(high_job));
                        unit_set_level(this, level-jobmaxLevel, None);
                        unit_update_weapon_mask(this, None);
                        this.set_internal_level(jobmaxLevel);
                    }
                }  
            }
        }
    }
}

//Adjusting stats
#[skyline::hook(offset = 0x01a0b1b0)]
pub fn autoGrowCap(this: &mut Unit, level: i32, targetLevel: i32, method_info: OptionalMethod){
    unsafe {
        //prevent the game from crashing at startup
        if !GROWTH_SET || this.person.name.get_string().unwrap() == "MPID_Lueur" { 
            call_original!(this, level, targetLevel, method_info);
            return; 
        }
        let player_cap = GameVariableManager::get_number("G_NG_CAP".into());
        if person_get_AssetForce(this.person, None) == 0 {
            let mut new_target = targetLevel + (targetLevel/4 as i32) + 1;
            // If RR and Mauvier or Vander then do not add extra levels
            if (is_reverse_recruitment() && this.person.name.get_string().unwrap() == "MPID_Mauve" ) || this.person.name.get_string().unwrap() == "MPID_Vandre" {
                call_original!(this, level, targetLevel, method_info);
                return;
            }
            call_original!(this, level, new_target, method_info);
            let jobmaxLevel = this.m_Job.MaxLevel as i32;
            //Promote recruited character 
            if jobmaxLevel < targetLevel {
                if job_is_low(this.m_Job, None) {
                    let high_job = job_get_high_job1(this.m_Job, None);
                    if !is_null_empty(high_job, None){
                        let hjob = JobData::get(&high_job.get_string().unwrap());
                        if hjob.is_some(){
                            let old_level = this.m_Job.MaxLevel as i32;
                            this.class_change(hjob.unwrap()); 
                            call_original!(this, level, new_target+1, method_info);
                            unit_update_weapon_mask(this, None);
                            let excessLevel:i32 = targetLevel - old_level;
                            unit_set_level(this, excessLevel.into(), None);
                        }
                    }
                }
            }
            let starting_cap = unit_cap_total(this, true);
            let mut enemy_cap = unit_cap_total(this, true);
            let mut count = 0;
            let countLimit = new_target / 5;
            let unit_level = this.m_Level;
            //Adjust Stats 
            while enemy_cap < player_cap && count < countLimit {
                Unit_LevelUP(this, 3, None);
                enemy_cap = unit_cap_total_mut(this, true);
                this.m_Level = unit_level;
                count += 1;
            }
            if starting_cap != enemy_cap { println!("Player {} {} gain {} stat points to {}", get_str(this.m_Job.name), get_str(this.person.name), enemy_cap-starting_cap, enemy_cap); }
            return;
        }
        let diff = GameUserData::get_difficulty(false);
        let mut new_enemy_Level = GetAverageLevel(2, 14 - 2*diff, None) - 4 + diff*2;
    
        if player_cap < 120 || !GameVariableManager::get_bool("G_Cleared_M006".into()) { 
            // FX and FX Skrimishes
            if player_cap < 0 { call_original!(this, level, targetLevel+2*diff, method_info); }
            // Skrimishes
            else if player_cap == 0 { call_original!(this, level, targetLevel, method_info); }
            // Divine Paralogues and Firene Arc
            else if targetLevel < new_enemy_Level {
                let new_level = new_enemy_Level - targetLevel + level;
                call_original!(this, new_level, new_enemy_Level, method_info);
                promote_unit(this, new_level);
            }
            else { call_original!(this, level, targetLevel, method_info); }
            unit_cap_total_mut(this, true);
            return; 
        }

        if is_boss(this.person){ new_enemy_Level += 4; }
        else if !Capability_is_zero(get_Grow(this.person, None), None) { new_enemy_Level += 2;} 

        if targetLevel < new_enemy_Level  { 
            let new_level = new_enemy_Level - targetLevel + level;
            call_original!(this, new_level, new_enemy_Level, method_info); 
            promote_unit(this, new_level);
            unit_cap_total_mut(this, true);
        }
        else { call_original!(this, level, targetLevel, method_info);  }
        //Adjusting Enemy Rating 
        if person_get_AssetForce(this.person, None) == 1 {
            let starting_cap = unit_cap_total(this, true);
            let mut enemy_cap = unit_cap_total(this, true);
            let mut count = 0;
            let unit_level = this.m_Level;
            let enemy_floor_cap = player_cap + diff*( get_number_main_chapters_completed() - 10 );
            while enemy_cap < enemy_floor_cap && count < 20 {
                Unit_LevelUP(this, 4, None);
                enemy_cap = unit_cap_total_mut(this, true);
                this.m_Level = unit_level;
                count += 1;
            }
            if this.person.name.get_string().unwrap() == "MPID_SombreDragon" { return; }
            let mut down_count = 0;
            while enemy_cap > player_cap+75*diff+25 && down_count < 50 {
                this.m_Level = unit_level+1;
                Unit_LevelDown(this, None);
                enemy_cap = unit_cap_total_mut(this, true);
                down_count += 1;
            }
            if starting_cap != enemy_cap { 
                println!("Enemy {} {} gain {} stat points to {} ( {} Up/ {} Down )", get_str(this.m_Job.name), get_str(this.person.name), enemy_cap-starting_cap, enemy_cap, count, down_count);
             }
            unit_set_Hp(this, unit_get_capability(this, 0, true, None), None);
            let jobmaxLevel = this.m_Job.MaxLevel;
            let unit_internal = this.m_InternalLevel;
            if jobmaxLevel < this.m_Level {
                let excessLevel: i8 = this.m_Level as i8 - jobmaxLevel as i8;
                this.set_internal_level((unit_internal + excessLevel).into());
                unit_set_level(this, jobmaxLevel.into(), None);
            }
        }
    }
}
//Hook to change/update weapons, adding engraves and suchs
#[skyline::hook(offset = 0x01a08de0)]
pub fn create_from_dispos(this: &mut Unit, data: &DisposData, method_info: OptionalMethod){
    call_original!(this, data, method_info);
    unsafe {
        if GameVariableManager::get_number("G_NG_CAP".into()) < 120 { return; }
        let is_NG_p = GameVariableManager::get_bool("G_NG");
        let personIndex: usize  = find_personIndex(this.person.pid);
        let mut refine_level = 0;   // Forging Level
        let mut level_difference: i32 = 0;
        let list_count = UnitItemList_Get_Count(this.m_ItemList, None);
        if personIndex != 0 {
            if autolevel::CLASS_LEVEL[personIndex] < 20  {
                // promoted so change out weapons
                if !job_is_low(this.m_Job, None) && this.m_Job.MaxLevel == 20 {
                    level_difference = this.m_Level as i32 - 20;
                    for x in 0..list_count {
                        let mut item = UnitItemList_Get_Item(this.m_ItemList, x, None);
                        if item.is_some() {
                            replace_weapon(item.as_mut().unwrap());
                            let weapon = &item.unwrap();
                        }
                    }
                }
                else { level_difference = this.m_Level as i32 - INITIAL_LEVEL[personIndex] as i32; }
                refine_level = level_difference / 4; 
            }
            else { 
                level_difference = this.m_Level as i32 - INITIAL_LEVEL[personIndex] as i32;
                refine_level = level_difference / 6; 
            }
                
                // Forging and adding engraves (if NG+)
                if refine_level < 0 { refine_level = 0; }
                if refine_level > 4 { refine_level = 5; }
                for x in 0..list_count {
                    let mut item = UnitItemList_Get_Item(this.m_ItemList, x, None);
                    if item.is_some() {
                        let weapon = &item.unwrap();
                        if UnitItem_IsExistRefineData(weapon, None) { UnitItem_Set_RefineLevel(weapon, refine_level, None);}
                        if is_NG_p && UnitItem_IsWeapon(weapon, None) {
                            let rng = random_getMinMax(random_get_Game(None), 0, 24, None) as usize;
                            if rng < 12 {
                                let godData = &GodData::get_list_mut().expect("triabolical is 'None'").list.items[rng];
                                UnitItem_SetEngrave(weapon, godData, None);
                       }
                    }
                }
            }
        }
    }
}