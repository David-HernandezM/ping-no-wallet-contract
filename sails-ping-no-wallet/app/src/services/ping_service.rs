use sails_rtl::{
    gstd::{
        gservice,
        msg
    },
    prelude::*
};

use crate::state_global_variables::{
        ping_state::ping_state_mut,
        signless_accounts::{signless_accounts_state_ref, SignlessError}
    };

pub type UserAddress = ActorId;
pub type NoWalletAccount = String;
pub type UserData = (Option<UserAddress>, Option<NoWalletAccount>);

#[derive(Default)]
pub struct PingService;

#[gservice]
impl PingService {
    pub fn new() -> Self {
        Self
    }

    pub fn ping(
        &mut self,
        user_data: UserData
    ) -> PingEvent {
        let ping_state = ping_state_mut();
        let signless_state = signless_accounts_state_ref();
        let (user_address, no_wallet_account) = user_data;

        let caller: ActorId = msg::source().into();
        
        let last_who_call;

        if user_address.is_some() {
            let address = match signless_state.get_user_address(caller, user_address) {
                Ok(address) => address,
                Err(error_message) => {
                    return PingEvent::Error(error_message);
                }
            };

            last_who_call = address;
        } else if no_wallet_account.is_some() {
            if let Err(error_message) = signless_state
                .check_signless_address_by_no_wallet_account(
                    caller, 
                    no_wallet_account.unwrap()
                ) {
                return PingEvent::Error(error_message);
            }

            last_who_call = caller;
        } else {
            last_who_call = caller;
        }

        ping_state.last_who_call = last_who_call;
        
        PingEvent::Pong
    }

    pub fn pong(
        &mut self,
        user_data: UserData
    ) -> PingEvent {
        let ping_state = ping_state_mut();
        let signless_state = signless_accounts_state_ref();
        let (user_address, no_wallet_account) = user_data;

        let caller: ActorId = msg::source().into();
        
        let last_who_call;

        if user_address.is_some() {
            let address = match signless_state.get_user_address(caller, user_address) {
                Ok(address) => address,
                Err(error_message) => {
                    return PingEvent::Error(error_message);
                }
            };

            last_who_call = address;
        } else if no_wallet_account.is_some() {
            if let Err(error_message) = signless_state
                .check_signless_address_by_no_wallet_account(
                    caller, 
                    no_wallet_account.unwrap()
                ) {
                return PingEvent::Error(error_message);
            }

            last_who_call = caller;
        } else {
            last_who_call = caller;
        }

        ping_state.last_who_call = last_who_call;
        
        PingEvent::Ping
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum PingEvent {
    Ping,
    Pong,
    Error(SignlessError)
}