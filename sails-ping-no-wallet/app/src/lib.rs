#![no_std]

use sails_rtl::{
    prelude::*,
    gstd::{
        gprogram, 
        groute, 
        msg
    }, 
    cell::RefCell
};

pub mod state_types;
pub mod services;

use state_types::{
    ping_state_type::PingState,
    signless_accounts_state_type::ContractSignlessAccounts
};

use services::{
    ping_service::PingService,
    signless_service::SignlessService,
    query_service::QueryService
};

pub struct PingProgram {
    ping_state: RefCell<PingState>,
    signless_state: RefCell<ContractSignlessAccounts>
}

#[gprogram]
impl PingProgram {
    pub fn new() -> Self {
        // unsafe {
        //     PING_STATE = Some(
        //         PingState {
        //             last_who_call: msg::source().into()
        //         }
        //     );
        //     SIGNLESS_ACCOUNTS = Some(
        //         ContractSignlessAccounts {
        //             signless_accounts_address_by_user_address: BTreeMap::new(),
        //             signless_accounts_address_by_no_wallet_name: BTreeMap::new(),
        //             signless_data_by_signless_address: BTreeMap::new()
        //         }
        //     );
        // };

        let ping_state = RefCell::new(PingState {
            last_who_call: msg::source().into()
        });
        let signless_state = RefCell::new(
            ContractSignlessAccounts::default()
        );

        Self {
            ping_state,
            signless_state
        }
    }

    #[groute("Ping")]
    pub fn ping_svc(&self) -> PingService<'_, '_> {
        PingService::new(
            self.ping_state.borrow_mut(),
            self.signless_state.borrow()
        )
    }

    #[groute("Signless")]
    pub fn signless_svc(&self) -> SignlessService<'_> {
        SignlessService::new(
            self.signless_state.borrow_mut()
        )
    }

    #[groute("Query")]
    pub fn query_svc(&self) -> QueryService<'_, '_> {
        QueryService::new(
            self.ping_state.borrow(),
            self.signless_state.borrow()
        )
    }
}