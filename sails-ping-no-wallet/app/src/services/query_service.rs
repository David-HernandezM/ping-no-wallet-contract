use sails_rtl::{
    gstd::gservice,
    cell::Ref,
    prelude::*
};

use crate::state_types::{
    signless_accounts_state_type::{
        // signless_accounts_state_ref,
        ContractSignlessAccounts,
        SignlessAccount
    },
    ping_state_type::PingState
    // ping_state_type::ping_state_ref
};

// #[derive(Default)]
pub struct QueryService<'a, 'b> {
    ping_state_ref: Ref<'a, PingState>,
    signless_state_ref: Ref<'b, ContractSignlessAccounts>
}

#[gservice]
impl<'a, 'b> QueryService<'a, 'b> {
    pub fn new(
        ping_state_ref: Ref<'a, PingState>,
        signless_state_ref: Ref<'b, ContractSignlessAccounts>
    ) -> Self {
        Self {
            ping_state_ref,
            signless_state_ref
        }
    }

    pub fn last_who_call(&self) -> QueryEvent {
        // let ping_state = ping_state_ref();

        QueryEvent::LastWhoCall(self.ping_state_ref.last_who_call)
    }

    pub fn signless_address_from_user_address(
        &self,
        user_address: ActorId
    ) -> QueryEvent {
        // let signless_state = signless_accounts_state_ref();

        // let signless_address = signless_state
        //     .signless_accounts_address_by_user_address
        //     .get(&user_address);

        let signless_address = self.signless_state_ref
            .signless_accounts_address_by_user_address
            .get(&user_address);

        QueryEvent::SignlessAccountAddress(signless_address.copied())
    }

    pub fn signless_address_from_no_wallet_account(
        &self,
        no_wallet_account: String
    ) -> QueryEvent {
        // let signless_state = signless_accounts_state_ref();

        // let signless_address = signless_state
        //     .signless_accounts_address_by_no_wallet_name
        //     .get(&no_wallet_account);

        let signless_address = self.signless_state_ref
            .signless_accounts_address_by_no_wallet_name
            .get(&no_wallet_account);

        QueryEvent::SignlessAccountAddress(signless_address.copied())
    }

    pub fn signless_account_data(
        &self,
        signless_address: ActorId
    ) -> QueryEvent {
        // let signless_state = signless_accounts_state_ref();

        // let signless_data = signless_state
        //     .signless_data_by_signless_address
        //     .get(&signless_address);

        let signless_data = self.signless_state_ref
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