use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*, system::*};
use engage::{proc::{ProcInstFields, Bindable}, singleton::SingletonProcInst};
use engage::proc::desc::ProcDesc;
use engage::gamedata::item::*;
use engage::gamevariable::GameVariableManager;
use engage::gameuserdata::GameUserData;

#[unity::class("App", "Random")]
pub struct Random { }

#[unity::from_offset("App", "Random", ".ctor")]
pub fn Random_ctor(this: &Random, seed: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x293a700)]
pub fn get_IsItemReturn(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2939950)]
pub fn get_well_useFlag(method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "WellSequence", "get_ExchangeLevel")]
pub fn get_well_exchangeLevel(method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "WellSequence", "get_Seed")]
pub fn get_well_seed(method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x2939a80)]
pub fn set_well_flag(value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x2939dc0)]
pub fn set_well_level(value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x293a100)]
pub fn set_seed(value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "WellSequence", "GetItem")]
pub fn well_get_item(this: &u64, method_info: OptionalMethod);

#[unity::from_offset("App", "WellSequence", "CalcItemExchange")]
pub fn well_CalcItemExchange(this: &u64, level: i32, random: &Random, method_info: OptionalMethod) -> &'static List<ItemData> ;

#[skyline::from_offset(0x293ac80)]
pub fn well_CreateBind(this: &u64, method_info: OptionalMethod);


#[skyline::from_offset(0x0203edf0)]
pub fn item_gain_create_bind(super_ : &u64, item: &ItemData, count: i32, method_info: OptionalMethod);

