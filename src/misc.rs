use unity::prelude::*;
use engage::menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}};
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use skyline::patching::Patch;
use engage::force::Force;
use crate::engage_functions::*;
use crate::autolevel::*;
pub const AUTOLEVEL_KEY: &str = "G_AUTOLEVEL";
// Misc things

//Toggle for Autoleveling bench
pub struct AutolevelMod;
impl ConfigBasicMenuItemSwitchMethods for AutolevelMod {
    fn init_content(this: &mut ConfigBasicMenuItem){
        GameVariableManager::make_entry_norewind(AUTOLEVEL_KEY, 0);
    }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_bool(AUTOLEVEL_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(toggle);

        if toggle != result {
            GameVariableManager::set_bool(AUTOLEVEL_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_bool(AUTOLEVEL_KEY);
        if !typeC {this.help_text = format!("Units will not be autoleveled at the end of the chapter.").into(); }
        else {this.help_text = format!("Low level units will be autoleveled at the end of the chapter.").into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C =  GameVariableManager::get_bool(AUTOLEVEL_KEY);
        if !type_C {this.command_text = format!("Off").into(); }
        else {this.command_text  = format!("On").into(); }
    }
}
extern "C" fn auto() -> &'static mut ConfigBasicMenuItem { ConfigBasicMenuItem::new_switch::<AutolevelMod>("Autolevel Bench Units") }
pub fn auto_install(){ cobapi::install_game_setting(auto); }

//Bypassing the internal level limit in the unit$$class change until Level 99
#[skyline::hook(offset=0x01a3c7b0)]
pub fn classChange(this: & mut Unit, job: &JobData, item: &u8, method_info: OptionalMethod){
    let internal_level = this.m_InternalLevel;
    let current_level = this.m_Level;
    let mut newLevel = 1;
    let mut newInternal = 0;
    unsafe {
        if job_is_low(this.m_Job, None) && jobdata_get_max_level(this.m_Job, None) == 99 {
            if !job_is_low(job, None){
                newLevel = current_level;
                newInternal = internal_level;
            }
            else if jobdata_get_max_level(job, None) == 99 && job_is_low(job, None) {
                newLevel = current_level;
                newInternal = internal_level;
            }
            else if job_is_low(job, None) && jobdata_get_max_level(job, None) == 20 {
                newLevel = 1;
                newInternal = current_level - 1 + internal_level;
            }
        }
        else if job_is_low(this.m_Job, None) && jobdata_get_max_level(this.m_Job, None) == 20 {
            if !job_is_low(job, None) {
                newLevel = 1;
                newInternal = current_level - 1 + internal_level;
            }
            else {
                newLevel = current_level;
                newInternal = internal_level;
                if newLevel == 20 && (job_is_low(job, None) && jobdata_get_max_level(job, None) == 20) {
                    newLevel = 1;
                    newInternal = current_level - 1 + internal_level;
                }
            }
        }
        else if !job_is_low(this.m_Job, None){
            if job_is_low(job, None) && jobdata_get_max_level(job, None) == 20 {
                newLevel = 1;
                newInternal = current_level - 1 + internal_level;
            }
            else {
                newLevel = current_level;
                newInternal = internal_level;
            }
        }
        if newLevel == 99 {
            newLevel = 21;
            newInternal = newInternal + 99 - 22;
        }
        if newInternal > 99 { newInternal = 99; }
        call_original!(this, job, item, method_info);
        this.m_Level = newLevel;
        this.m_InternalLevel = newInternal;
    }
}
//NG+ bypassing Engrave limit
#[skyline::hook(offset=0x0295ce30)]
pub fn create_engrave(this: u64, method_info: OptionalMethod){
    if GameVariableManager::get_bool("G_NG")|| GameVariableManager::get_bool("G_Cleared_M022") { Patch::in_text(0x0295d5c8).bytes([0x00, 0x00, 0x80, 0xD2]).unwrap(); }
    else { Patch::in_text(0x0295d5c8).bytes([0xB2, 0xE1, 0xD8, 0x97]).unwrap(); }
    call_original!(this, method_info);
}

//removing void curse on enemy :(
#[skyline::hook(offset = 0x01a0b1b0)]
pub fn autoGrowCap(this: &mut Unit, level: i32, target_level: i32, method_info: OptionalMethod){
    call_original!(this, level, target_level, method_info);
    unsafe {
        if person_get_AssetForce(this.person, None) != 0 {
            let total_level = this.m_Level + this.m_InternalLevel as u8;
            this.m_Level = total_level;
            this.m_InternalLevel = 0;
        } 
        let void_curse: &str = "SID_虚無の呪い";
        unit_RemoveEquipSkill(this, void_curse.into(), None);
        unit_removeEquipSkillPool(this, void_curse.into(), None);
        unit_removePrivateSkill(this, void_curse.into(), None);
    }
}

//When Chapter is completed to get ignots and well
#[skyline::hook(offset = 0x02513620)]
pub fn get_ignots(this: &GameUserData, method_info: OptionalMethod){
    call_original!(this, None);
    updateIgnots();
}

//Auto Ignots and well
pub fn updateIgnots(){
    unsafe {
        let instance = GameUserData::get_instance();
        let ironCount = 110 + get_iron(instance, None) + 10*get_number_of_chapters_completed();
        let steelCount = 11 + get_steel(instance, None);
        let silverCount = 3 + get_silver(instance, None);
        let mut BondCount = 0;
        if GameVariableManager::get_bool("G_NG") { BondCount = 2500 + get_PieceOfBond(instance, None) + 100*get_number_of_chapters_completed(); }
        else { BondCount = get_PieceOfBond(instance, None) + 100*get_number_of_chapters_completed(); }
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
                set_well_level(3, None);
                set_well_flag(2, None);
                let seed = 4*(ironCount + steelCount + silverCount + 1750);
                set_seed(seed, None);
            }
            else if get_well_exchangeLevel(None) < 3 { 
                if GameVariableManager::get_bool("G_NG") { set_well_level(4, None); }
                else { set_well_level(2, None); }
            }
        }
        if GameVariableManager::get_bool("G_NG"){ 
            let typeC = GameVariableManager::get_number("G_NG_OPTION");
            if typeC == 1 { autolevel_party(10, 4, false); }
            else if typeC >= 2 { autolevel_party(10, 2, false); }
        }
        else if GameVariableManager::get_bool(AUTOLEVEL_KEY) { 
            autolevel_party(10, 5, false); 
            println!("Bench is autoleveled");
        }
    }
}

//Ignore Mauvier for Reverse Recruitment
#[skyline::hook(offset=0x01cd5f30)]
pub fn ignoreJagens(this: u64, unit: &Unit, method_info: OptionalMethod){
    unsafe {
        let pid = unit_get_pid(unit, None);
        if pid.get_string().unwrap() == "PID_モーヴ" && is_reverse_recruitment() { return; }
        else { call_original!(this, unit, method_info); }
    }
}
#[skyline::hook(offset=0x01cd6020)]
pub fn ignoreMauvierLevel(this: u64, unit: &Unit, method_info: OptionalMethod){
    unsafe {
        let pid = unit_get_pid(unit, None);
        if pid.get_string().unwrap() == "PID_モーヴ" && is_reverse_recruitment() { return;  }
        else { call_original!(this, unit, method_info); }
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
#[skyline::hook(offset=0x02b414f0)]
pub fn is_recollection(this: u64, method_info: OptionalMethod){
    call_original!(this, method_info);
    if GameVariableManager::get_bool("G_NG"){
        unsafe {
            let instance = GameUserData::get_instance();
            let status = get_UserData_Status(instance, None);
            status.value = 8;
        }
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

#[skyline::hook(offset=0x02199cb0)]
pub fn join_unit_check(arg: u64, index: i32, method_info: OptionalMethod) -> Option<&'static PersonData> {
    unsafe {
    let d_type = get_dynType(arg, index, None);
    if d_type == 0 { call_original!(arg, index, method_info) }
    else if d_type == 4 {
        let pid = try_get_string(arg, index, "nothing".into(), method_info);
        if !is_recruited(pid) { call_original!(arg, index, method_info) }
        else { return None; }
    }
    else {
        call_original!(arg, index, method_info)
    }
}

}
