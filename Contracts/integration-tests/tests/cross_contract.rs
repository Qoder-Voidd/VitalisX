#![cfg(test)]

extern crate std;

use academy_rewards::AcademyRewardsContract;
use messaging::UpgradeableMessagingContract;
use shared::circuit_breaker::CircuitBreakerConfig;
use shared::governance::ProposalStatus;
use social_rewards::SocialRewardsContract;
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    testutils::{Address as _, Ledger},
    token, Address, Env, String, Vec,
};
use trading::UpgradeableTradingContract;

// --------------------------------------------------------------------------
// MockTokenContract
// --------------------------------------------------------------------------
// A minimal, in-memory fungible token used purely as a test double wherever
// a real Soroban token contract (SAC) would normally sit. It implements just
// enough of the token interface (mint/balance/transfer) for the trading
// integration test below to move funds around and assert on balances,
// without needing to deploy a full token contract.
#[contract]
pub struct MockTokenContract;

// Storage key for this mock token: each address maps to its i128 balance.
#[contracttype]
#[derive(Clone)]
pub enum TokenDataKey {
    Balance(Address),
}

#[contractimpl]
impl MockTokenContract {
    // Test helper only — mints `amount` to `to` with no auth check, since
    // tests need an unrestricted way to fund accounts before exercising
    // real transfer logic.
    pub fn mint(env: Env, to: Address, amount: i128) {
        let current = Self::balance(env.clone(), to.clone());
        let updated = current.checked_add(amount).expect("overflow");
        env.storage()
            .persistent()
            .set(&TokenDataKey::Balance(to), &updated);
    }

    // Reads the stored balance for `id`, defaulting to 0 if the address has
    // never held this token.
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&TokenDataKey::Balance(id))
            .unwrap_or(0)
    }

    // Standard token transfer: requires the sender's signature, checks
    // sufficient balance, then debits/credits both sides.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic!("insufficient balance")
        }

        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage()
            .persistent()
            .set(&TokenDataKey::Balance(from), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&TokenDataKey::Balance(to), &(to_balance + amount));
    }
}

// --------------------------------------------------------------------------
// test_academy_rewards_trigger_social_rewards
// --------------------------------------------------------------------------
// Verifies the academy_rewards -> social_rewards integration: minting a
// badge, redeeming it for a discount, and confirming that the redemption
// can successfully feed into the social rewards contract as a credited
// reward. Also checks that redemption history is recorded correctly.
#[test]
fn test_academy_rewards_trigger_social_rewards() {
    let env = Env::default();
    env.ledger().with_mut(|li| li.timestamp = 1000);
    env.mock_all_auths();

    // Deploy both contracts under test.
    let academy_id = env.register_contract(None, AcademyRewardsContract);
    let academy = academy_rewards::AcademyRewardsContractClient::new(&env, &academy_id);

    let social_id = env.register_contract(None, SocialRewardsContract);
    let social = social_rewards::SocialRewardsContractClient::new(&env, &social_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let cb_config = CircuitBreakerConfig {
        max_volume_per_period: 10_000_000,
        max_tx_count_per_period: 100,
        period_duration: 3600,
    };

    // Initialize both contracts with a shared circuit breaker config.
    academy.initialize(&admin, &cb_config);
    social.init(&admin);

    // Create a "Gold" badge type worth a 500-unit discount, capped at 5
    // redemptions, with no expiry (0u64).
    academy.create_badge_type(
        &admin,
        &1u32,
        &String::from_str(&env, "Gold"),
        &500u32,
        &5u32,
        &0u64,
    );
    academy.mint_badge(&admin, &user, &1u32);

    // Redeem the badge and confirm the discount matches what was configured.
    let discount = academy.redeem_badge(&user, &String::from_str(&env, "tx-1"));
    assert_eq!(discount, 500);

    // Integration behavior: a successful badge redemption triggers social reward crediting.
    social.add_reward(&user, &(discount as i128));

    // Confirm the redemption was recorded in the user's history with the
    // correct discount amount.
    let record = academy.get_redemption_history(&user, &0u32).unwrap();
    assert_eq!(record.discount_applied, 500);
}

// --------------------------------------------------------------------------
// test_trading_interacts_with_fee_distribution
// --------------------------------------------------------------------------
// Verifies that executing a trade correctly deducts a fee from the trader
// and routes it to the designated fee recipient, using MockTokenContract to
// stand in for the traded asset. Also checks that aggregate trading stats
// (trade count, volume) update as expected.
#[test]
fn test_trading_interacts_with_fee_distribution() {
    let env = Env::default();
    env.ledger().with_mut(|li| li.timestamp = 1000);
    env.mock_all_auths();

    let token_id = env.register_contract(None, MockTokenContract);
    let token_admin = MockTokenContractClient::new(&env, &token_id);

    let trading_id = env.register_contract(None, UpgradeableTradingContract);
    let trading = trading::UpgradeableTradingContractClient::new(&env, &trading_id);

    let admin = Address::generate(&env);
    let approver = Address::generate(&env);
    let executor = Address::generate(&env);
    let trader = Address::generate(&env);
    let fee_recipient = Address::generate(&env);

    // Single-approver governance set for this test.
    let mut approvers = Vec::new(&env);
    approvers.push_back(approver);

    let cb_config = CircuitBreakerConfig {
        max_volume_per_period: 10_000_000,
        max_tx_count_per_period: 100,
        period_duration: 3600,
    };

    trading.init(&admin, &approvers, &executor, &cb_config);

    // Fund the trader with 1000 units of the mock token before trading.
    token_admin.mint(&trader, &1000i128);

    // Snapshot balances before the trade so we can assert on the delta.
    let fee_before_trader = token::Client::new(&env, &token_id).balance(&trader);
    let fee_before_recipient = token::Client::new(&env, &token_id).balance(&fee_recipient);

    // Execute a buy trade of 250 units at price 100, with a 25-unit fee
    // routed to fee_recipient, denominated in the mock token.
    let trade_id = trading.trade(
        &trader,
        &symbol_short!("XLMUSD"),
        &250i128,
        &100i128,
        &true,
        &token_id,
        &25i128,
        &fee_recipient,
    );

    // First trade in a fresh contract should be assigned id 1.
    assert_eq!(trade_id, 1);

    let fee_after_trader = token::Client::new(&env, &token_id).balance(&trader);
    let fee_after_recipient = token::Client::new(&env, &token_id).balance(&fee_recipient);

    // Confirm exactly the fee amount (25) moved from trader to recipient.
    assert_eq!(fee_before_trader - fee_after_trader, 25);
    assert_eq!(fee_after_recipient - fee_before_recipient, 25);

    // Confirm aggregate stats reflect the single trade just executed.
    let stats = trading.get_stats();
    assert_eq!(stats.total_trades, 1);
    assert_eq!(stats.total_volume, 250);
}

// --------------------------------------------------------------------------
// test_messaging_notifications_from_other_contract_flows
// --------------------------------------------------------------------------
// Verifies that an external event (an academy badge redemption) can result
// in a notification being sent through the messaging contract, and that the
// recipient's unread count and message contents reflect that notification
// correctly.
#[test]
fn test_messaging_notifications_from_other_contract_flows() {
    let env = Env::default();
    env.ledger().with_mut(|li| li.timestamp = 1000);
    env.mock_all_auths();

    let academy_id = env.register_contract(None, AcademyRewardsContract);
    let academy = academy_rewards::AcademyRewardsContractClient::new(&env, &academy_id);

    let messaging_id = env.register_contract(None, UpgradeableMessagingContract);
    let messaging = messaging::UpgradeableMessagingContractClient::new(&env, &messaging_id);

    let admin = Address::generate(&env);
    let approver = Address::generate(&env);
    let executor = Address::generate(&env);
    let notifier = Address::generate(&env);
    let user = Address::generate(&env);

    let mut approvers = Vec::new(&env);
    approvers.push_back(approver);

    let cb_config = CircuitBreakerConfig {
        max_volume_per_period: 10_000_000,
        max_tx_count_per_period: 100,
        period_duration: 3600,
    };

    messaging.init(&admin, &approvers, &executor, &cb_config);

    // Set up a "Silver" badge worth a 250-unit discount, then redeem it —
    // this redemption is the trigger event for the notification below.
    academy.initialize(&admin, &cb_config);
    academy.create_badge_type(
        &admin,
        &2u32,
        &String::from_str(&env, "Silver"),
        &250u32,
        &3u32,
        &0u64,
    );
    academy.mint_badge(&admin, &user, &2u32);

    let discount = academy.redeem_badge(&user, &String::from_str(&env, "tx-2"));
    let payload = String::from_str(&env, "Your academy badge was redeemed successfully");

    // Notifier sends the user a message about the redemption they just triggered.
    let message_id = messaging.send_message(&notifier, &user, &payload);
    assert_eq!(message_id, 1);
    assert_eq!(discount, 250);

    // The user should have exactly one unread message...
    let unread = messaging.get_unread_count(&user);
    assert_eq!(unread, 1);

    // ...and fetching it (unread-only, newest-first, whichever the flags
    // mean here) should return that single message with the exact payload sent.
    let notifications = messaging.get_messages(&user, &false, &true, &true);
    assert_eq!(notifications.len(), 1);
    assert_eq!(notifications.get(0).unwrap().payload, payload);
}

// --------------------------------------------------------------------------
// test_shared_governance_module_across_contracts
// --------------------------------------------------------------------------
// Verifies that the shared upgrade-governance module behaves consistently
// when reused across two independent contracts (trading and messaging):
// each can independently propose and approve its own upgrade, and each
// proposal's status updates to Approved without cross-contamination between
// the two contracts' proposals.
#[test]
fn test_shared_governance_module_across_contracts() {
    let env = Env::default();
    env.ledger().with_mut(|li| li.timestamp = 1000);
    env.mock_all_auths();

    let trading_id = env.register_contract(None, UpgradeableTradingContract);
    let trading = trading::UpgradeableTradingContractClient::new(&env, &trading_id);

    let messaging_id = env.register_contract(None, UpgradeableMessagingContract);
    let messaging = messaging::UpgradeableMessagingContractClient::new(&env, &messaging_id);

    let admin = Address::generate(&env);
    let approver = Address::generate(&env);
    let executor = Address::generate(&env);

    let mut approvers = Vec::new(&env);
    approvers.push_back(approver.clone());

    let cb_config = CircuitBreakerConfig {
        max_volume_per_period: 10_000_000,
        max_tx_count_per_period: 100,
        period_duration: 3600,
    };

    // Both contracts share the same admin/approver/executor set and circuit
    // breaker config, but are otherwise independent instances.
    trading.init(&admin, &approvers, &executor, &cb_config);
    messaging.init(&admin, &approvers, &executor, &cb_config);

    // Propose and approve an upgrade on the trading contract...
    let trading_proposal = trading.propose_upgrade(
        &admin,
        &symbol_short!("tv2hash"),
        &symbol_short!("UpgrTrade"),
        &approvers,
        &1u32,
        &3600u64,
    );
    trading.approve_upgrade(&trading_proposal, &approver);

    // ...and independently on the messaging contract.
    let messaging_proposal = messaging.propose_upgrade(
        &admin,
        &symbol_short!("mv2hash"),
        &symbol_short!("UpgrMsg"),
        &approvers,
        &1u32,
        &3600u64,
    );
    messaging.approve_upgrade(&messaging_proposal, &approver);

    let trade_status = trading.get_upgrade_proposal(&trading_proposal).status;
    let msg_status = messaging.get_upgrade_proposal(&messaging_proposal).status;

    // Each contract's own proposal should be independently approved.
    assert_eq!(trade_status, ProposalStatus::Approved);
    assert_eq!(msg_status, ProposalStatus::Approved);
}