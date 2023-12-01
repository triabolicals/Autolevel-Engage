use skyline::patching::Patch;
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::*;
use engage::gamedata::person::Capability;

//Functions from the game 

//Capability Functions
#[skyline::from_offset(0x25bcd00)]
pub fn Capability_add(this: &Capability, i: i32, v: u8, method_info: OptionalMethod);

#[skyline::from_offset(0x25bcda0)]
pub fn Capability_is_zero(this: &Capability, method_info: OptionalMethod) -> bool;

//PersonData Functions
#[skyline::from_offset(0x1f26140)]
pub fn person_get_combat_bgm(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;


//JobData Functions
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


//Well Functions
#[skyline::from_offset(0x293a700)]
pub fn get_IsItemReturn(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2939a80)]
pub fn set_well_flag(value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x2939dc0)]
pub fn set_well_level(value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x293a100)]
pub fn set_seed(value: i32, method_info: OptionalMethod);

//Other
//Check if Il2CppString is empty
#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

//Get Average Level of Party
#[skyline::from_offset(0x2b4afa0)]
pub fn GetAverageLevel(difficulty: i32, sortieCount: i32, method_info: OptionalMethod) -> i32;
