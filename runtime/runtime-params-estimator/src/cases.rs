use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::{TryFrom, TryInto};

use num_rational::Ratio;

use near_crypto::{InMemorySigner, KeyType, PublicKey};
use near_primitives::account::{AccessKey, AccessKeyPermission, FunctionCallPermission};
use near_primitives::hash::CryptoHash;
use near_primitives::transaction::{
    Action, AddKeyAction, CreateAccountAction, DeleteAccountAction, DeleteKeyAction,
    DeployContractAction, FunctionCallAction, SignedTransaction, StakeAction, TransferAction,
};

use crate::ext_costs_generator::ExtCostsGenerator;
use crate::runtime_fees_generator::RuntimeFeesGenerator;
use crate::stats::Measurements;
use crate::testbed::RuntimeTestbed;
use crate::testbed_runners::GasMetric;
use crate::testbed_runners::{get_account_id, measure_actions, measure_transactions, Config};
use crate::vm_estimator::{cost_per_op, cost_to_compile};
use near_runtime_fees::{
    AccessKeyCreationConfig, ActionCreationConfig, DataReceiptCreationConfig, Fee,
    RuntimeFeesConfig,
};
use near_vm_logic::{ExtCosts, ExtCostsConfig, VMConfig, VMKind, VMLimitConfig};
use node_runtime::config::RuntimeConfig;

/// How much gas there is in a nanosecond worth of computation.
const GAS_IN_MEASURE_UNIT: u128 = 1_000_000u128;

fn measure_function(
    metric: Metric,
    method_name: &'static str,
    measurements: &mut Measurements,
    testbed: RuntimeTestbed,
    accounts_deployed: &[usize],
    nonces: &mut HashMap<usize, u64>,
    config: &Config,
    allow_failures: bool,
    args: Vec<u8>,
) -> RuntimeTestbed {
    // Measure the speed of creating a function fixture with 1MiB input.
    let mut rng = rand_xorshift::XorShiftRng::from_seed([0u8; 16]);
    let mut f = || {
        let account_idx = *accounts_deployed.choose(&mut rng).unwrap();
        let account_id = get_account_id(account_idx);
        let signer = InMemorySigner::from_seed(&account_id, KeyType::ED25519, &account_id);
        let nonce = *nonces.entry(account_idx).and_modify(|x| *x += 1).or_insert(1);
        let function_call = Action::FunctionCall(FunctionCallAction {
            method_name: method_name.to_string(),
            args: args.clone(),
            gas: 10u64.pow(18),
            deposit: 0,
        });
        SignedTransaction::from_actions(
            nonce as u64,
            account_id.clone(),
            account_id,
            &signer,
            vec![function_call],
            CryptoHash::default(),
        )
    };
    measure_transactions(metric, measurements, config, Some(testbed), &mut f, allow_failures)
}

macro_rules! calls_helper(
    { $($el:ident => $method_name:ident),* } => {
    {
        let mut v: Vec<(Metric, &str)> = vec![];
        $(
            v.push((Metric::$el, stringify!($method_name)));
        )*
        v
    }
    };
);

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Metric {
    Receipt,
    ActionTransfer,
    ActionCreateAccount,
    ActionDeleteAccount,
    ActionAddFullAccessKey,
    ActionAddFunctionAccessKey1Method,
    ActionAddFunctionAccessKey1000Methods,
    ActionDeleteAccessKey,
    ActionStake,
    ActionDeploy10K,
    ActionDeploy100K,
    ActionDeploy1M,

    warmup,
    noop_1MiB,
    noop,
    base_1M,
    read_memory_10b_10k,
    read_memory_1Mib_10k,
    write_memory_10b_10k,
    write_memory_1Mib_10k,
    read_register_10b_10k,
    read_register_1Mib_10k,
    write_register_10b_10k,
    write_register_1Mib_10k,
    utf8_log_10b_10k,
    utf8_log_10kib_10k,
    nul_utf8_log_10b_10k,
    nul_utf8_log_10kib_10k,
    utf16_log_10b_10k,
    utf16_log_10kib_10k,
    nul_utf16_log_10b_10k,
    nul_utf16_log_10kib_10k,
    sha256_10b_10k,
    sha256_10kib_10k,
    keccak256_10b_10k,
    keccak256_10kib_10k,
    keccak512_10b_10k,
    keccak512_10kib_10k,
    storage_write_10b_key_10b_value_1k,
    storage_write_10kib_key_10b_value_1k,
    storage_write_10b_key_10kib_value_1k,
    storage_write_10b_key_10kib_value_1k_evict,
    storage_read_10b_key_10b_value_1k,
    storage_read_10kib_key_10b_value_1k,
    storage_read_10b_key_10kib_value_1k,
    storage_remove_10b_key_10b_value_1k,
    storage_remove_10kib_key_10b_value_1k,
    storage_remove_10b_key_10kib_value_1k,
    storage_has_key_10b_key_10b_value_1k,
    storage_has_key_10kib_key_10b_value_1k,
    storage_has_key_10b_key_10kib_value_1k,

    promise_and_100k,
    promise_and_100k_on_1k_and,
    promise_return_100k,
    data_producer_10b,
    data_producer_100kib,
    data_receipt_10b_1000,
    data_receipt_100kib_1000,
    cpu_ram_soak_test,
}

pub fn run(mut config: Config) -> RuntimeConfig {
    let mut m = Measurements::new(config.metric);
    config.block_sizes = vec![100];

    // Measure the speed of creating account.
    let mut nonces: HashMap<usize, u64> = HashMap::new();
    let mut f = || {
        let account_idx = rand::thread_rng().gen::<usize>() % config.active_accounts;
        let account_id = get_account_id(account_idx);
        let other_account_id = format!("random_account_{}", rand::thread_rng().gen::<usize>());
        let signer = InMemorySigner::from_seed(&account_id, KeyType::ED25519, &account_id);
        let nonce = *nonces.entry(account_idx).and_modify(|x| *x += 1).or_insert(1);
        SignedTransaction::from_actions(
            nonce as u64,
            account_id,
            other_account_id,
            &signer,
            vec![
                Action::CreateAccount(CreateAccountAction {}),
                Action::Transfer(TransferAction { deposit: 10u128.pow(26) }),
            ],
            CryptoHash::default(),
        )
    };

    // Measure the speed of deleting an account.
    let mut nonces: HashMap<usize, u64> = HashMap::new();
    let mut deleted_accounts = HashSet::new();
    let mut beneficiaries = HashSet::new();
    let mut f = || {
        let account_idx = loop {
            let x = rand::thread_rng().gen::<usize>() % config.active_accounts;
            if !deleted_accounts.contains(&x) & &!beneficiaries.contains(&x) {
                break x;
            }
        };
        let beneficiary_idx = loop {
            let x = rand::thread_rng().gen::<usize>() % config.active_accounts;
            if !deleted_accounts.contains(&x) && x != account_idx {
                break x;
            }
        };
        deleted_accounts.insert(account_idx);
        beneficiaries.insert(beneficiary_idx);
        let account_id = get_account_id(account_idx);
        let beneficiary_id = get_account_id(beneficiary_idx);
        let signer = InMemorySigner::from_seed(&account_id, KeyType::ED25519, &account_id);
        let nonce = *nonces.entry(account_idx).and_modify(|x| *x += 1).or_insert(1);
        SignedTransaction::from_actions(
            nonce as u64,
            account_id.clone(),
            account_id,
            &signer,
            vec![Action::DeleteAccount(DeleteAccountAction { beneficiary_id })],
            CryptoHash::default(),
        )
    };

    // Measure the speed of adding an access key with 1k methods each 10bytes long.
    let many_methods: Vec<_> = (0..1000).map(|i| format!("a123456{:03}", i)).collect();
    measure_actions(
        Metric::ActionAddFunctionAccessKey1000Methods,
        &mut m,
        &config,
        None,
        vec![Action::AddKey(AddKeyAction {
            public_key: serde_json::from_str(
                "\"ed25519:DcA2MzgpJbrUATQLLceocVckhhAqrkingax4oJ9kZ847\"",
            )
            .unwrap(),
            access_key: AccessKey {
                nonce: 0,
                permission: AccessKeyPermission::FunctionCall(FunctionCallPermission {
                    allowance: Some(100),
                    receiver_id: get_account_id(0),
                    method_names: many_methods,
                }),
            },
        })],
        true,
        true,
    );

    get_runtime_config(&m)

    //    let mut csv_path = PathBuf::from(&config.state_dump_path);
    //    csv_path.push("./metrics.csv");
    //    m.save_to_csv(csv_path.as_path());
    //
    //    m.plot(PathBuf::from(&config.state_dump_path).as_path());
}

fn ratio_to_gas(gas_metric: GasMetric, value: Ratio<u64>) -> u64 {
    let divisor = match gas_metric {
        // We use factor of 8 to approximately match the price of SHA256 operation between
        // time-based and icount-based metric as measured on 3.2Ghz Core i5.
        GasMetric::ICount => 8u128,
        GasMetric::Time => 1u128,
    };
    u64::try_from(
        Ratio::<u128>::new(
            (*value.numer() as u128) * GAS_IN_MEASURE_UNIT,
            (*value.denom() as u128) * divisor,
        )
        .to_integer(),
    )
    .unwrap()
}

/// Converts cost of a certain action to a fee, spliting it evenly between send and execution fee.
fn measured_to_fee(gas_metric: GasMetric, value: Ratio<u64>) -> Fee {
    let value = ratio_to_gas(gas_metric, value);
    Fee { send_sir: value / 2, send_not_sir: value / 2, execution: value / 2 }
}

fn measured_to_gas(
    gas_metric: GasMetric,
    measured: &BTreeMap<ExtCosts, Ratio<u64>>,
    cost: ExtCosts,
) -> u64 {
    match measured.get(&cost) {
        Some(value) => ratio_to_gas(gas_metric, *value),
        None => panic!("cost {} not found", cost as u32),
    }
}

fn get_runtime_fees_config(measurement: &Measurements) -> RuntimeFeesConfig {
    use crate::runtime_fees_generator::ReceiptFees::*;
    let generator = RuntimeFeesGenerator::new(measurement);
    let measured = generator.compute();
    let metric = measurement.gas_metric;
    let mut rfc = RuntimeFeesConfig::default();

    rfc.action_creation_config.add_key_cost.function_call_cost_per_byte =
        measured_to_fee(metric, measured[&ActionAddFunctionAccessKeyPerByte]);
    rfc
}

fn get_ext_costs_config(measurement: &Measurements) -> ExtCostsConfig {
    let mut generator = ExtCostsGenerator::new(measurement);
    let measured = generator.compute();
    let metric = measurement.gas_metric;
    use ExtCosts::*;
    let (contract_compile_cost, contract_compile_base_cost) =
        cost_to_compile(metric, VMKind::default());
    ExtCostsConfig {
        base: measured_to_gas(metric, &measured, base),
        contract_compile_base: contract_compile_base_cost,
        contract_compile_bytes: ratio_to_gas(metric, contract_compile_cost),
        read_memory_base: measured_to_gas(metric, &measured, read_memory_base),
        read_memory_byte: measured_to_gas(metric, &measured, read_memory_byte),
        write_memory_base: measured_to_gas(metric, &measured, write_memory_base),
        write_memory_byte: measured_to_gas(metric, &measured, write_memory_byte),
        read_register_base: measured_to_gas(metric, &measured, read_register_base),
        read_register_byte: measured_to_gas(metric, &measured, read_register_byte),
        write_register_base: measured_to_gas(metric, &measured, write_register_base),
        write_register_byte: measured_to_gas(metric, &measured, write_register_byte),
        utf8_decoding_base: measured_to_gas(metric, &measured, utf8_decoding_base),
        utf8_decoding_byte: measured_to_gas(metric, &measured, utf8_decoding_byte),
        utf16_decoding_base: measured_to_gas(metric, &measured, utf16_decoding_base),
        utf16_decoding_byte: measured_to_gas(metric, &measured, utf16_decoding_byte),
        sha256_base: measured_to_gas(metric, &measured, sha256_base),
        sha256_byte: measured_to_gas(metric, &measured, sha256_byte),
        keccak256_base: measured_to_gas(metric, &measured, keccak256_base),
        keccak256_byte: measured_to_gas(metric, &measured, keccak256_byte),
        keccak512_base: measured_to_gas(metric, &measured, keccak512_base),
        keccak512_byte: measured_to_gas(metric, &measured, keccak512_byte),
        log_base: measured_to_gas(metric, &measured, log_base),
        log_byte: measured_to_gas(metric, &measured, log_byte),
        storage_write_base: measured_to_gas(metric, &measured, storage_write_base),
        storage_write_key_byte: measured_to_gas(metric, &measured, storage_write_key_byte),
        storage_write_value_byte: measured_to_gas(metric, &measured, storage_write_value_byte),
        storage_write_evicted_byte: measured_to_gas(metric, &measured, storage_write_evicted_byte),
        storage_read_base: measured_to_gas(metric, &measured, storage_read_base),
        storage_read_key_byte: measured_to_gas(metric, &measured, storage_read_key_byte),
        storage_read_value_byte: measured_to_gas(metric, &measured, storage_read_value_byte),
        storage_remove_base: measured_to_gas(metric, &measured, storage_remove_base),
        storage_remove_key_byte: measured_to_gas(metric, &measured, storage_remove_key_byte),
        storage_remove_ret_value_byte: measured_to_gas(
            metric,
            &measured,
            storage_remove_ret_value_byte,
        ),
        storage_has_key_base: measured_to_gas(metric, &measured, storage_has_key_base),
        storage_has_key_byte: measured_to_gas(metric, &measured, storage_has_key_byte),
        // TODO: storage_iter_* operations below are deprecated, so just hardcode zero price,
        // and remove those operations ASAP.
        storage_iter_create_prefix_base: 0,
        storage_iter_create_prefix_byte: 0,
        storage_iter_create_range_base: 0,
        storage_iter_create_from_byte: 0,
        storage_iter_create_to_byte: 0,
        storage_iter_next_base: 0,
        storage_iter_next_key_byte: 0,
        storage_iter_next_value_byte: 0,
        // TODO: Actually compute it once our storage is complete.
        // TODO: temporary value, as suggested by @nearmax, divisor is log_16(20000) ~ 3.57 ~ 7/2.
        touching_trie_node: measured_to_gas(metric, &measured, storage_read_base) * 2 / 7,
        promise_and_base: measured_to_gas(metric, &measured, promise_and_base),
        promise_and_per_promise: measured_to_gas(metric, &measured, promise_and_per_promise),
        promise_return: measured_to_gas(metric, &measured, promise_return),
        // TODO: accurately price host functions that expose validator information.
        validator_stake_base: 303944908800,
        validator_total_stake_base: 303944908800,
    }
}

fn get_vm_config(measurement: &Measurements) -> VMConfig {
    VMConfig {
        ext_costs: get_ext_costs_config(measurement),
        // TODO: Figure out whether we need this fee at all. If we do what should be the memory
        // growth cost.
        grow_mem_cost: 1,
        regular_op_cost: ratio_to_gas(measurement.gas_metric, cost_per_op(measurement.gas_metric))
            as u32,
        limit_config: VMLimitConfig::default(),
    }
}

fn get_runtime_config(measurement: &Measurements) -> RuntimeConfig {
    let mut runtime_config = RuntimeConfig::default();
    runtime_config.transaction_costs = get_runtime_fees_config(measurement);
    // runtime_config.wasm_config = get_vm_config(measurement);
    runtime_config
}
