use program_io::{ InitEscrow, EscrowAction, EscrowEvent, EscrowState };
use gtest::{ Log, Program, System };

const BUYER: u64 = 100;
const SELLER: u64 = 101;
const PRICE: u128 = 100_000;

const ESCROW_ID: u64 = 1;

#[test]
fn deposit() {
    let sys = System::new();
    init_escrow(&sys);

    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, PRICE);

    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        PRICE,
    );
    let log = Log::builder()
        .dest(BUYER)
        .payload(EscrowEvent::FundsDeposited);
    assert!(res.contains(&log));

    let escrow_balance = sys.balance_of(ESCROW_ID);
    assert_eq!(escrow_balance, PRICE);
}

#[test]
fn deposit_failures() {
    let sys = System::new();
    init_escrow(&sys);

    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, 2*PRICE);
    // must fail since BUYER attaches not enough value
    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        2*PRICE - 500,
    );
    assert!(res.main_failed());

    // must fail since the message sender is not BUYER
    let res = escrow.send(SELLER, EscrowAction::Deposit);
    assert!(res.main_failed());

    // successful deposit
    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        PRICE,
    );
    assert!(!res.main_failed());
      // must fail since the state must be `AwaitingPayment`
      let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        PRICE,
    );
    assert!(res.main_failed());
}

// Homework test
#[test]
fn confirm_delivery() {
    let sys = System::new();
    init_escrow(&sys);

    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, PRICE);

    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        PRICE,
    );

    let res = escrow.send(BUYER, EscrowAction::ConfirmDelivery);

    sys.claim_value_from_mailbox(SELLER);
    
    let seller_balance = sys.balance_of(SELLER);

    assert_eq!(seller_balance, PRICE);
}

// Homework test
#[test]
fn confirm_delivery_failures() {
    let sys = System::new();
    init_escrow(&sys);

    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, 2*PRICE);
    // must fail since BUYER attaches not enough value
    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        2*PRICE - 500,
    );
    assert!(res.main_failed());

    // must fail since the message sender is not BUYER
    let res = escrow.send(SELLER, EscrowAction::Deposit);
    assert!(res.main_failed());

    // successful deposit
    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        PRICE,
    );

    assert!(!res.main_failed());
    // must fail since the state must be `AwaitingPayment`
    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        PRICE,
    );
    assert!(res.main_failed());

    // must fail since the message sender is not Buyer
    let res = escrow.send(SELLER, EscrowAction::ConfirmDelivery);
    assert!(res.main_failed());

    // Successful confirm delivery
    let res = escrow.send(BUYER, EscrowAction::ConfirmDelivery);
    assert!(!res.main_failed());

    // must fail since the state is`Closed`
    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit,
        PRICE,
    );
    assert!(res.main_failed());

    // must fail since the state is`Closed`    
    let res = escrow.send(BUYER, EscrowAction::ConfirmDelivery);
    assert!(res.main_failed());
}

fn init_escrow(sys: &System) {
    sys.init_logger();
    let escrow = Program::current(&sys);
    let res = escrow.send(
        SELLER,
        InitEscrow {
            seller: SELLER.into(),
            buyer: BUYER.into(),
            price: PRICE,
        },
    );
    assert!(!res.main_failed());
}