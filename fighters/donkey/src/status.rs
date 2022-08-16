use super::*;
use globals::*;
// status script import
 
mod item_throw_heavy;
mod special_hi;
mod link_event;

unsafe extern "C" fn when_shield(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::get_stick_y(fighter.module_accessor) < 0.3 {
        
        // if you are alraedy holding an item and you do the barrel pull input, just cargo carry that item instead
        if (ItemModule::is_have_item(fighter.module_accessor, 0)) {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(),true.into());
            return true.into();
        }
        
        // otherwise, pull a barrel if the timer is up
        else if VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) == 0 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1);
            ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BARREL),0,0,false,false);
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(),true.into());
            return true.into();
        }
    }
    return false.into();
}
  
// setting the callback for shield to be used for b in shield
#[smashline::fighter_init]
fn donkey_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_DONKEY {
        fighter.global_table[0x34].assign(&L2CValue::Ptr(when_shield as *const () as _));
        }
    }
}

pub fn install() {
    item_throw_heavy::install();
    special_hi::install();
    link_event::install();
    smashline::install_agent_init_callbacks!(donkey_init);
}