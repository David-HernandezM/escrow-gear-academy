#![no_std]

use gear_lib::non_fungible_token::{
    state::NFTQueryReply,
    token::{Token, TokenId},
};
use gmeta::{metawasm, Metadata};
use gstd::{ActorId, Vec, prelude::*};
use program_io::{
    ProgramMetadata,
    EscrowState
};

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn seller(state: State) -> ActorId {
        state.seller
    }

    pub fn buyer(state: State) -> ActorId {
        state.buyer
    }

    pub fn escrow_state(state: State) -> EscrowState {
        state.state
    }
}

