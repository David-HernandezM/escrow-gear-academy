use gear_lib::non_fungible_token::{io::NFTTransfer, nft_core::*, state::*, token::*};
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gmeta::Metadata;
use gstd::{errors::Result as GstdResult, exec, msg, prelude::*, ActorId, MessageId};
use hashbrown::HashMap;
use primitive_types::{H256, U256};

use program_io::{
    ProgramMetadata,
    InitEscrow,
    EscrowAction,
    EscrowState,
    Escrow
};

static mut ESCROW: Option<Escrow> = None;

fn state_mut() -> &'static mut Escrow {
    let state = unsafe { ESCROW.as_mut() };

    debug_assert!(state.is_some(), "state isn't initialized");

    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let init_config: InitEscrow = msg::load()
        .expect("Error in decoding `InitEscrow");
    let escrow = Escrow {
        seller: init_config.seller,
        buyer: init_config.buyer,
        price: init_config.price,
        state: EscrowState::AwaitingPayment
    };
    ESCROW = Some(escrow);
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: EscrowAction = msg::load()
        .expect("Unable to decode `EscrowAction");
    let escrow = state_mut();
    match action {
        EscrowAction::Deposit => escrow.deposit(),
        EscrowAction::ConfirmDelivery => escrow.confirm_delivery()
    }
}

#[no_mangle]
extern "C" fn state() {
    let escrow = state_mut();
    msg::reply(escrow, 0).expect("Failed to share state");
}