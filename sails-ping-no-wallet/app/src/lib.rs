#![no_std]

use sails_rtl::{
    collections::BTreeMap, gstd::{
        gprogram, groute, msg
    }, prelude::*
};

pub mod state_global_variables;
pub mod services;

use state_global_variables::{
    ping_state::{
        PING_STATE,
        PingState
    },
    signless_accounts::{
        SIGNLESS_ACCOUNTS,
        ContractSignlessAccounts
    }
};

use services::{
    ping_service::PingService,
    signless_service::SignlessService
};

#[derive(Default)]
pub struct PingProgram;

#[gprogram]
impl PingProgram {
    pub fn new() -> Self {
        unsafe {
            PING_STATE = Some(
                PingState {
                    last_who_call: msg::source().into()
                }
            );
            SIGNLESS_ACCOUNTS = Some(
                ContractSignlessAccounts {
                    signless_accounts_address_by_user_address: BTreeMap::new(),
                    signless_accounts_address_by_no_wallet_name: BTreeMap::new(),
                    signless_data_by_signless_address: BTreeMap::new()
                }
            );
        };

        Self
    }

    #[groute("Ping")]
    pub fn ping_svc(&self) -> PingService {
        PingService::new()
    }

    #[groute("Signless")]
    pub fn signless_svc(&self) -> SignlessService {
        SignlessService::new()
    }

}