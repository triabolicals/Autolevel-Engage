use skyline::patching::Patch;
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*, system::*};
use engage::gamedata::*;
use engage::{force::*, gamevariable::*, gameuserdata::*, gamedata::unit::*};
use engage::gamedata::person::Capability;
use engage::gamedata::person::SkillArray;
//Functions from the game 

//Capability Functions
#[skyline::from_offset(0x25bcd00)]
pub fn Capability_add(this: &Capability, i: i32, v: u8, method_info: OptionalMethod);

#[skyline::from_offset(0x25bcda0)]
pub fn Capability_is_zero(this: &Capability, method_info: OptionalMethod) -> bool;

#[unity::class("App", "CapabilitySbyte")]
pub struct CapabilitySbyte {}

#[skyline::from_offset(0x25bdf90)]
pub fn CapabilitySbyte_add(this: &CapabilitySbyte, i: i32, v: u8,  method_info: OptionalMethod);

//SkillArray
#[skyline::from_offset(0x02482850)]
pub fn skillarray_remove(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> bool;

//PersonData Functions
#[skyline::from_offset(0x1f26140)]
pub fn person_get_combat_bgm(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

//JobData Functions
#[unity::from_offset("App", "JobData", "get_DiffGrowLunatic")]
pub fn job_get_DiffGrowL(this: &JobData, method_info: OptionalMethod) -> &CapabilitySbyte;

#[unity::from_offset("App", "JobData", "get_DiffGrowHard")]
pub fn job_get_DiffGrowH(this: &JobData, method_info: OptionalMethod) -> &CapabilitySbyte;

#[unity::from_offset("App", "JobData", "get_DiffGrowNormal")]
pub fn job_get_DiffGrowN(this: &JobData, method_info: OptionalMethod) -> &CapabilitySbyte;

#[unity::from_offset("App", "JobData", "get_DiffGrow")]
pub fn job_get_DiffGrow(this: &JobData, method_info: OptionalMethod) -> &CapabilitySbyte;

#[unity::from_offset("App", "JobData", "set_DiffGrowLunatic")]
pub fn job_set_DiffGrowL(this: &JobData,value: &CapabilitySbyte, method_info: OptionalMethod);

#[unity::from_offset("App", "JobData", "set_DiffGrowHard")]
pub fn job_set_DiffGrowH(this: &JobData, value: &CapabilitySbyte, method_info: OptionalMethod);

#[unity::from_offset("App", "JobData", "set_DiffGrowNormal")]
pub fn job_set_DiffGrowN(this: &JobData, value: &CapabilitySbyte, method_info: OptionalMethod);

#[unity::from_offset("App", "JobData", "set_DiffGrow")]
pub fn job_set_DiffGrow(this: &JobData, value: &CapabilitySbyte,  method_info: OptionalMethod);

#[skyline::from_offset(0x2055d70)]
pub fn job_has_high_job(this: &JobData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2053ea0)]
pub fn get_job_internal_level(this: &JobData, method_info: OptionalMethod) -> u8;

#[skyline::from_offset(0x2055d20)]
pub fn job_is_low(this: &JobData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2054ac0)]
pub fn job_get_low(this: &JobData, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x2053e80)]
pub fn job_max_level(this: &JobData, method_info: OptionalMethod) -> u8;

#[skyline::from_offset(0x1f25c70)]
pub fn person_set_Jid(this: &PersonData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x2054980)]
pub fn job_get_high_job1(this: &JobData, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x2054a20)]
pub fn job_get_high_job2(this: &JobData, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x2055fe0)]
pub fn job_GetLowJobs(this: &JobData, method_info: OptionalMethod) -> &List<JobData>;

//Job Weapons
#[unity::from_offset("App", "JobData", "get_WeaponAxe")]
pub fn job_getWeaponAxe(this: &JobData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App", "JobData", "get_WeaponBow")]
pub fn job_getWeaponBow(this: &JobData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App", "JobData", "get_WeaponDagger")]
pub fn job_getWeaponDagger(this: &JobData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App", "JobData", "get_WeaponFist")]
pub fn job_getWeaponFist(this: &JobData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App", "JobData", "get_WeaponLance")]
pub fn job_getWeaponLance(this: &JobData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App", "JobData", "get_WeaponRod")]
pub fn job_getWeaponRod(this: &JobData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App", "JobData", "get_WeaponSpecial")]
pub fn job_getWeaponSpecial(this: &JobData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App", "JobData", "get_WeaponSword")]
pub fn job_getWeaponSword(this: &JobData, method_info: OptionalMethod) -> i8;

//Chapter Data
#[unity::from_offset("App", "ChapterData", "set_RecommendedLevel")]
pub fn chapter_set_recommended_level(this: &ChapterData, value: u8, method_info: OptionalMethod);

#[unity::from_offset("App", "ChapterData", "get_RecommendedLevel")]
pub fn chapter_get_recommended_level(this: &ChapterData, method_info: OptionalMethod) -> u8;

//Other
//Check if Il2CppString is empty
#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x3784700)]
pub fn string_start_with(this: &Il2CppString, value: &Il2CppString, method_info: OptionalMethod) -> bool;

pub fn str_start_with(this: &Il2CppString, value: &str) -> bool {
   unsafe { string_start_with(this, value.into(), None) }
}

//Get Average Level of Party
#[skyline::from_offset(0x2b4afa0)]
pub fn GetAverageLevel(difficulty: i32, sortieCount: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x1f25e60)]
pub fn person_get_AssetForce(this: &PersonData, method_info: OptionalMethod) -> i32;

//Function that does the level up
#[skyline::from_offset(0x01a3a040)]
pub fn Unit_LevelUP(this: &Unit, abort: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x02616200)]
pub fn Force_Get(forceType: i32, method_info: OptionalMethod) -> &'static Force;

#[skyline::from_offset(0x02af9850)]
pub fn chapter_set_flag(this: &ChapterData, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "AddSkillPoint")]
pub fn unit_add_SP(this: &Unit, value: i32, method_info: OptionalMethod);


// Random Functions
#[unity::class("App", "Random")]
pub struct Random {}

#[unity::from_offset("App", "Random", "get_Game")]
pub fn random_get_Game(method_info: OptionalMethod) -> &'static Random;

#[skyline::from_offset(0x023751b0)]
pub fn random_getMinMax(this: &Random, min: i32, max: i32, method_info: OptionalMethod) -> i32;
