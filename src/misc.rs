use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use skyline::patching::Patch;
use engage::force::Force;
use crate::engage_functions::*;
use crate::autolevel::*;
use crate::autolevel;
use unity::il2cpp::object::Array;
pub const AUTOLEVEL_KEY: &str = "G_AUTOLEVEL";
// Misc things

//Toggle for Autoleveling bench
pub struct AutolevelMod;
impl ConfigBasicMenuItemSwitchMethods for AutolevelMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ GameVariableManager::make_entry(AUTOLEVEL_KEY, 0); } 
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_bool(AUTOLEVEL_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);
        if toggle != result {
            GameVariableManager::set_bool(AUTOLEVEL_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else { return BasicMenuResult::new();  }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if ! GameVariableManager::get_bool(AUTOLEVEL_KEY) { this.help_text = "Units will not be autoleveled at the end of the chapter.".into();  }
        else { this.help_text = "Low level units will be autoleveled at the end of the chapter.".into();   }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if !GameVariableManager::get_bool(AUTOLEVEL_KEY) { this.command_text = Off_str();  }
        else { this.command_text  = On_str();}
    }
}
extern "C" fn auto() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<AutolevelMod>("Autolevel Bench")   }
pub fn auto_install(){ cobapi::install_game_setting(auto); }

//Bypassing the internal level limit in the unit$$class change until Level 99
#[skyline::hook(offset=0x01a3c7b0)]
pub fn classChange(this: & mut Unit, job: &JobData, item: &u8, method_info: OptionalMethod){
    let internal_level = this.m_InternalLevel;
    let current_level = this.m_Level;
    let mut newLevel = 1;
    let mut newInternal: i32 = 0;
    unsafe {
        let old_type = autolevel::CLASS_TYPE[ JobData::get_index(this.m_Job.jid) as usize ];
        let new_type = autolevel::CLASS_TYPE[ JobData::get_index(job.jid) as usize ];

        //Special + Promoted 
        if old_type >= 1 {
            //to Special/Promoted 
            if new_type >= 1 {
                newLevel = current_level;
                newInternal = internal_level as i32;
                if newLevel >= 99 {
                    newLevel = 21;
                    newInternal = 78 + internal_level as i32;
                }
            }
            else if old_type == 2 && current_level < 20 {   // Special Class to Base Class less base class level cap
                newLevel = current_level;
                newInternal = internal_level as i32;
            }
            else {  //to base
                newLevel = 1;
                newInternal = current_level as i32 + internal_level as i32 - 1;
            }
        }
        // Base 
        else {
            if (new_type == 2 || new_type == 0) && current_level < 20 { // to special/base and less than base class level cap
                newLevel = current_level;
                newInternal = internal_level as i32;
            }
            else {            // to promoted/special
                newLevel = 1;
                newInternal = current_level as i32 + internal_level as i32 - 1;
            }
        }
        if newInternal > 99 { newInternal = 99; }
        call_original!(this, job, item, method_info);
        this.m_Level = newLevel;
        this.m_InternalLevel = newInternal as i8;
        if JobLearnSkill(job, None) <= newLevel.into() { LearnJobSkill_Unit(this, None); } 
    }
}
#[unity::from_offset("App", "HubFacilityData", "SetFirstAccessFlag")]
fn hub_facility_set_access_flag(this: &HubFacilityData, method_info: OptionalMethod);

//When Chapter is completed to get ignots and well
//activate dragon ride when chapter 11 ends
#[skyline::hook(offset = 0x02513620)]
pub fn get_ignots(this: &GameUserData, method_info: OptionalMethod){
    call_original!(this, None);
    updateIgnots();
    let dragon_ride = HubFacilityData::get("AID_ドラゴンライド").unwrap();
    if GameVariableManager::get_bool("G_Cleared_M011") && dragon_ride.is_complete() {
       unsafe { hub_facility_set_access_flag(dragon_ride, None); }
    }
    
}
//Auto Ignots and well
pub fn updateIgnots(){
    unsafe {
        let instance = GameUserData::get_instance();
        let ironCount = 110 + get_iron(instance, None) + 10*get_number_of_chapters_completed();
        let mut steelCount = 11 + get_steel(instance, None);
        let mut silverCount = 3 + get_silver(instance, None);
        let mut BondCount = get_PieceOfBond(instance, None) + 100*get_number_of_chapters_completed();
        if GameVariableManager::get_bool("G_NG") {
            BondCount += 2500;
            steelCount += 10;
            silverCount += 5;
        }
        set_PieceOfBond(instance, BondCount, None);
        println!("Bond Fragments: {}", BondCount);
        if GameVariableManager::get_bool("G_拠点_動物小屋通知") {
            set_iron(instance, ironCount, None);
            set_steel(instance, steelCount, None);
            set_silver(instance, silverCount, None);
            println!("Ingot: {} Iron, {} Steel, {} Silver", ironCount, steelCount, silverCount);
        }
        if GameVariableManager::get_number("G_拠点_裏武器イベント") > 1 {
            let can_get_items = get_IsItemReturn(None);
            if can_get_items == false {
                if GameVariableManager::get_bool("G_NG") { set_well_level(4, None) }
                else if GameVariableManager::get_bool("G_Cleared_E006"){ set_well_level(3, None); }
                else { set_well_level(2, None); }
                let seed = 4*(ironCount + steelCount + silverCount + 1750);
                set_seed(seed, None);
                set_well_flag(2, None);
            }
            else if get_well_exchangeLevel(None) < 2 { 
                if GameVariableManager::get_bool("G_NG") { set_well_level(4, None) }
                else if GameVariableManager::get_bool("G_Cleared_E006"){ set_well_level(3, None); }
                else { set_well_level(2, None); }
            }
        }
        if GameVariableManager::get_bool(AUTOLEVEL_KEY) { 
            if GameVariableManager::get_bool("G_NG"){ 
                let typeC = GameVariableManager::get_number("G_NG_OPTION");
                if typeC == 1 { autolevel_party(10, 4, false); }
                else if typeC >= 2 { autolevel_party(10, 3, false); }
            }
            else { autolevel_party(10, 4, false);  }
            println!("Bench is autoleveled");
        }
    }
}
// Keep Job Skill at the same level when switching to level 99 cap
#[skyline::hook(offset=0x02056ca0)]
pub fn JobLearnSkill(this: &JobData, method_info: OptionalMethod) -> i32 {
    unsafe {
        let max_level = jobdata_get_max_level(this, None);
        if job_is_low(this, None) && max_level == 99 { return 25; }
        else if !job_is_low(this, None) { return 5; }
        return 25;
    }
}
pub fn get_number_of_chapters_completed() -> i32 {
    let mut number = 0;
    let chapters = ChapterData::get_list_mut().expect(":D");
    unsafe {
        let length = chapters.len();
        let game_variable = GameUserData::get_variable();
        for x in 0..length {
            if str_start_with(chapters[x].cid, "CID_M") || str_start_with(chapters[x].cid, "CID_S") || str_start_with(chapters[x].cid, "CID_G") || str_start_with(chapters[x].cid, "CID_E"){
                if get_bool(game_variable, GetClearedFlagName(chapters[x], None), None) { number = number + 1; }
            }
        }
    }
    number
}
pub fn get_number_main_chapters_completed() -> i32 {
    let mut number = 0;
    let chapters = ChapterData::get_list_mut().expect(":D");
    unsafe {
        let length = chapters.len();
        let game_variable = GameUserData::get_variable();
        for x in 0..length {
            if str_start_with(chapters[x].cid, "CID_M") || str_start_with(chapters[x].cid, "CID_S"){
                if get_bool(game_variable, GetClearedFlagName(chapters[x], None), None) { number = number + 1; }
            }
        }
    }
    number
}
pub fn is_recruited(pid: &Il2CppString) -> bool {
    unsafe {
        if is_null_empty(pid, None) { return false; }
        for force in 0..7 {
            if force == 1 || force == 2 { continue; }
            let benchForce = Force_Get(force, None);
            let mut force_iter = Force::iter(benchForce);
            while let Some(unit) = force_iter.next() {
                if pid.get_string().unwrap() == unit.person.pid.get_string().unwrap() {
                    println!("{} is already recruited", unit.person.name.get_string().unwrap() );
                    return true;
                }
            }
        }
    }
    false
}
#[skyline::from_offset(0x021a3620)]
pub fn get_dynType(arg: u64, index: i32,  method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "ScriptUtil", "TryGetString")]
pub fn try_get_string(arg: u64, index: i32, nothing: &Il2CppString, method_info: OptionalMethod) -> &Il2CppString;

//Prevent units from joining twice
#[skyline::hook(offset=0x02199cb0)]
pub fn join_unit_check(arg: u64, index: i32, method_info: OptionalMethod) -> Option<&'static PersonData> {
    unsafe {
        let d_type = get_dynType(arg, index, None);
        if d_type == 0 { call_original!(arg, index, method_info) }
        else if d_type == 4 {
            let pid = try_get_string(arg, index, "nothing".into(), method_info);
            if pid.get_string().unwrap() == "PID_リュール" {
                call_original!(arg, index, method_info)
            }
            else if !is_recruited(pid) { call_original!(arg, index, method_info) }
            else { return None; }
        }
        else { call_original!(arg, index, method_info) }
    }
}
#[skyline::from_offset(0x025c9240)]
pub fn Mess_Get(label: &Il2CppString, method_info: OptionalMethod) -> &'static Il2CppString;

pub fn get_str(label: &Il2CppString) -> String { unsafe { Mess_Get(label, None).get_string().unwrap()  } }
pub fn On_str() -> &'static Il2CppString { unsafe { Mess_Get("MID_CONFIG_TUTORIAL_ON".into(), None) } }
pub fn Off_str() -> &'static Il2CppString { unsafe { Mess_Get("MID_CONFIG_TUTORIAL_OFF".into(), None) } }

//Removing Void Curse with opponent always hit
#[unity::hook("App", "PersonData", "set_CommonSids")]
pub fn set_sid(this: &PersonData, value: &mut Array<&Il2CppString>, method_info: OptionalMethod ){
    for i in 0..value.len() {
        if value[i].get_string().unwrap() ==  "SID_虚無の呪い" {
            value[i] = "SID_相手の命中１００".into();
        }
    }
    call_original!(this, value, method_info);
}