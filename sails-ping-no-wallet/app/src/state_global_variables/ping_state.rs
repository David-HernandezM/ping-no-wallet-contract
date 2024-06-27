use sails_rtl::prelude::*;

pub static mut PING_STATE: Option<PingState> = None;

pub struct PingState {
    pub last_who_call: ActorId
}

pub fn ping_state_mut() -> &'static mut PingState {
    let state = unsafe { PING_STATE.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

pub fn ping_state_ref() -> &'static PingState {
    let state = unsafe { PING_STATE.as_ref() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}