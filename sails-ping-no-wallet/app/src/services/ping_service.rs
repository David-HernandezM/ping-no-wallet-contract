use sails_rtl::{
    gstd::{
        gservice,
        msg
    },
    cell::{RefMut, Ref},
    prelude::*
};

use crate::state_types::{
    signless_accounts_state_type::{
        ContractSignlessAccounts,
        SignlessError
    },
    ping_state_type::PingState
};

pub type UserAddress = ActorId;
pub type NoWalletAccount = String;
pub type UserData = (Option<UserAddress>, Option<NoWalletAccount>);

// #[derive(Default)]
pub struct PingService<'a, 'b> {
    ping_state: RefMut<'a, PingState>,
    signless_state_ref: Ref<'b, ContractSignlessAccounts>
}

#[gservice]
impl<'a, 'b> PingService<'a, 'b> {
    pub fn new(
        ping_state: RefMut<'a, PingState>, 
        signless_state_ref: Ref<'b, ContractSignlessAccounts>
    ) -> Self {
        Self {
            ping_state,
            signless_state_ref
        }
    }

    pub fn ping(
        &mut self,
        user_data: UserData
    ) -> PingEvent {
        // let ping_state = ping_state_mut();
        // let signless_state = signless_accounts_state_ref();

        let (user_address, no_wallet_account) = user_data;

        let caller: ActorId = msg::source().into();
        
        let last_who_call;

        if user_address.is_some() {
            // let address = match signless_state.get_user_address(caller, user_address) {
            let address = match self.signless_state_ref.get_user_address(caller, user_address) {
                Ok(address) => address,
                Err(error_message) => {
                    return PingEvent::Error(error_message);
                }
            };

            last_who_call = address;
        } else if no_wallet_account.is_some() {
            // if let Err(error_message) = signless_state
            if let Err(error_message) = self.signless_state_ref
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

        // ping_state.last_who_call = last_who_call;
        self.ping_state.last_who_call = last_who_call;
        
        PingEvent::Pong
    }

    pub fn pong(
        &mut self,
        user_data: UserData
    ) -> PingEvent {
        // let ping_state = ping_state_mut();
        // let signless_state = signless_accounts_state_ref();

        let (user_address, no_wallet_account) = user_data;

        let caller: ActorId = msg::source().into();
        
        let last_who_call;

        if user_address.is_some() {
            // let address = match signless_state.get_user_address(caller, user_address) {
            let address = match self.signless_state_ref.get_user_address(caller, user_address) {
                Ok(address) => address,
                Err(error_message) => {
                    return PingEvent::Error(error_message);
                }
            };

            last_who_call = address;
        } else if no_wallet_account.is_some() {
            // if let Err(error_message) = signless_state
            if let Err(error_message) = self.signless_state_ref
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

        self.ping_state.last_who_call = last_who_call;
        
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