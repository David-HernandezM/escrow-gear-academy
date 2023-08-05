#![no_std]

use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    royalties::*,
    state::NFTState,
    token::*,
};
use gmeta::{In, InOut, Out, Metadata};
use gstd::{prelude::*, ActorId, msg};

use primitive_types::H256;

pub struct ProgramMetadata;

#[derive(Debug, Default, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum EscrowState {
    #[default]
    AwaitingPayment,
    AwaitingDelivery,
    Closed
}

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowEvent {
    FundsDeposited,
    DeliveryConfirmed,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowAction {
    Deposit,
    ConfirmDelivery 
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Escrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
    pub state: EscrowState,
}

impl Escrow {
    pub fn deposit(&mut self) {
        assert_eq!(
            self.state, 
            EscrowState::AwaitingPayment, 
            "State must be `AwaitingPayment"
        );
        assert_eq!(
            msg::source(),
            self.buyer,
            "The message sender must be a buyer"
        );
        assert_eq!(
            msg::value(),
            self.price,
            "The attached value must be equal to set price"
        );
        self.state = EscrowState::AwaitingDelivery;
        msg::reply(EscrowEvent::FundsDeposited, 0)
            .expect("Error in reply EscrowEvent::FundsDeposited");
    }

    pub fn confirm_delivery(&mut self) {
        assert_eq!(
            msg::source(),
            self.buyer,
            "The message sender must be a buyer"
        );
        assert_eq!(
            self.state,
            EscrowState::AwaitingDelivery,
            "State must be `AwaitingDelivery`"
        );
        msg::send(self.seller, "Buyer payment", self.price).expect("Error while sending funds");
        self.state = EscrowState::Closed;
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitEscrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
}

impl Metadata for ProgramMetadata {
    type Init = In<InitEscrow>;
    type Handle = InOut<EscrowAction, EscrowEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Escrow;
}
