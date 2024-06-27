use sails_rtl::{
    gstd::gservice,
    prelude::*
};

use crate::state_global_variables::{
    signless_accounts::{
        signless_accounts_state_ref,
        SignlessAccount
    },
    ping_state::ping_state_ref
};

#[derive(Default)]
pub struct QueryService;

#[gservice]
impl QueryService {
    pub fn new() -> Self {
        Self
    }

    pub fn last_who_call(&self) -> QueryEvent {
        let ping_state = ping_state_ref();

        QueryEvent::LastWhoCall(ping_state.last_who_call)
    }

    pub fn signless_address_from_user_address(
        &self,
        user_address: ActorId
    ) -> QueryEvent {
        let signless_state = signless_accounts_state_ref();

        let signless_address = signless_state
            .signless_accounts_address_by_user_address
            .get(&user_address);

        QueryEvent::SignlessAccountAddress(signless_address.copied())
    }

    pub fn signless_address_from_no_wallet_account(
        &self,
        no_wallet_account: String
    ) -> QueryEvent {
        let signless_state = signless_accounts_state_ref();

        let signless_address = signless_state
            .signless_accounts_address_by_no_wallet_name
            .get(&no_wallet_account);

        QueryEvent::SignlessAccountAddress(signless_address.copied())
    }

    pub fn signless_account_data(
        &self,
        signless_address: ActorId
    ) -> QueryEvent {
        let signless_state = signless_accounts_state_ref();

        let signless_data = signless_state
            .signless_data_by_signless_address
            .get(&signless_address);

        let response = match signless_data {
            Some(data) => Some(data.clone()),
            None => None
        };

        QueryEvent::SignlessAccountData(response)
    }

}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum QueryEvent {
    LastWhoCall(ActorId),
    SignlessAccountAddress(Option<ActorId>),
    SignlessAccountData(Option<SignlessAccount>)
}