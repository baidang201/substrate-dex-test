use super::*;
pub use crate::mock::{Event, ExchangeModule, ExtBuilder, Origin, System, Tokens, ALICE, BOB};
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

const ENDOWED_AMOUNT: u128 = 1_000_000_000_000_000;

#[derive(Encode, Decode, Clone, RuntimeDebug, Eq, PartialEq)]
pub struct Order<CurrencyId, Balance, AccountId> {
    pub base_currency_id: CurrencyId,
    #[codec(compact)]
    pub base_amount: Balance,
    pub target_currency_id: CurrencyId,
    #[codec(compact)]
    pub target_amount: Balance,
    pub owner: AccountId,
}

fn new_test_ext() -> sp_io::TestExternalities {
    let mut ext = ExtBuilder::default().build();
    ext.execute_with(|| System::set_block_number(1));
    ext
}

#[test]
fn test_submit_order() {
    new_test_ext().execute_with(|| {
        //sell amount <= balance
        assert_ok!(ExchangeModule::submit_order(
            Origin::signed(ALICE),
            DOT,
            10,
            BTC,
            1
        ));

        assert_eq!(Tokens::free_balance(DOT, &ALICE), ENDOWED_AMOUNT - 10);
        assert_eq!(Tokens::free_balance(BTC, &ALICE), ENDOWED_AMOUNT);
    });
}

#[test]
fn test_take_order() {
    new_test_ext().execute_with(|| {
        //id not exist
        assert_noop!(
            ExchangeModule::take_order(Origin::signed(BOB), 0),
            Error::<Test>::InvalidOrderId
        );

        //id exist
        assert_ok!(ExchangeModule::submit_order(
            Origin::signed(ALICE),
            DOT,
            10,
            BTC,
            1
        ));
        assert_eq!(Tokens::free_balance(DOT, &ALICE), ENDOWED_AMOUNT - 10);
        assert_eq!(Tokens::free_balance(BTC, &ALICE), ENDOWED_AMOUNT);

        assert_ok!(ExchangeModule::take_order(Origin::signed(BOB), 0));
        assert_eq!(Tokens::free_balance(DOT, &ALICE), ENDOWED_AMOUNT - 10);
        assert_eq!(Tokens::free_balance(BTC, &ALICE), ENDOWED_AMOUNT + 1);
        assert_eq!(Tokens::free_balance(DOT, &BOB), ENDOWED_AMOUNT + 10);
        assert_eq!(Tokens::free_balance(BTC, &BOB), ENDOWED_AMOUNT - 1);
    });
}

#[test]
fn test_cancel_order() {
    new_test_ext().execute_with(|| {
        //id not exist
        assert_noop!(
            ExchangeModule::cancel_order(Origin::signed(ALICE), 0),
            Error::<Test>::InvalidOrderId
        );

        //id exist, it is not owner
        assert_ok!(ExchangeModule::submit_order(
            Origin::signed(ALICE),
            DOT,
            10,
            BTC,
            1
        ));

        assert_noop!(
            ExchangeModule::cancel_order(Origin::signed(BOB), 0),
            Error::<Test>::NotOwner
        );

        //id exist, is owner
        assert_ok!(ExchangeModule::cancel_order(Origin::signed(ALICE), 0));
    });
}
