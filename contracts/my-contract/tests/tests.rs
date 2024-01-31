use concordium_smart_contract_testing::*;
use my_contract;

/// A test account.
const ALICE: AccountAddress = AccountAddress([0u8; 32]);
const ALICE_ADDR: Address = Address::Account(ALICE);

/// The initial balance of the ALICE test account.
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(10_000);

/// A [`Signer`] with one set of keys, used for signing transactions.
const SIGNER: Signer = Signer::with_one_key();

/// Test invoking the `increment` endpoint succeeds in updating the contract.
#[test]
fn test_increment_succeeds() {
    let (mut chain, init) = initialize();

    // Construct the parameter.
    let parameter = my_contract::IncrementParameter { increment_by: 10 };

    // Update the contract via the `increment` entrypoint with the parameter.
    chain
        .contract_update(
            SIGNER,
            ALICE,
            ALICE_ADDR,
            Energy::from(10_000),
            UpdateContractPayload {
                address: init.contract_address,
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("my_contract.increment".to_string()),
                message: OwnedParameter::from_serial(&parameter)
                    .expect("Parameter within size bounds"),
            },
        )
        .expect("Increment succeeds.");

    let invoke_view = chain
        .contract_invoke(
            ALICE,
            ALICE_ADDR,
            Energy::from(10_000),
            UpdateContractPayload {
                address: init.contract_address,
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("my_contract.view".to_string()),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Invoke 'view'");
    let state: my_contract::State = invoke_view
        .parse_return_value()
        .expect("Parsing return value of 'view'");

    assert_eq!(state.count, 10)
}

/// Test invoking the `increment` endpoint rejects, when increment is zero.
#[test]
fn test_increment_rejects() {
    let (mut chain, init) = initialize();

    // Construct the parameter.
    let parameter = my_contract::IncrementParameter { increment_by: 0 };

    // Update the contract via the `receive` entrypoint with the parameter `true`.
    let update = chain
        .contract_update(
            SIGNER,
            ALICE,
            ALICE_ADDR,
            Energy::from(10_000),
            UpdateContractPayload {
                address: init.contract_address,
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("my_contract.increment".to_string()),
                message: OwnedParameter::from_serial(&parameter)
                    .expect("Parameter within size bounds"),
            },
        )
        .expect_err("Update rejects with increment of zero.");

    // Check that the contract returned `YourError`.
    let error: my_contract::ContractError = update
        .parse_return_value()
        .expect("Deserialize `ContractError`");
    assert_eq!(error, my_contract::ContractError::IncrementByZero);
}

/// Helper method for initializing the contract.
///
/// Does the following:
///  - Creates the [`Chain`]
///  - Creates one account, `Alice` with `10_000` CCD as the initial balance.
///  - Initializes the contract.
///  - Returns the [`Chain`] and the [`ContractInitSuccess`]
fn initialize() -> (Chain, ContractInitSuccess) {
    // Initialize the test chain.
    let mut chain = Chain::new();

    // Create the test account.
    chain.create_account(Account::new(ALICE, ACC_INITIAL_BALANCE));

    // Load the module.
    let module = module_load_v1("./concordium-out/module.wasm.v1").expect("Module exists at path");
    // Deploy the module.
    let deployment = chain
        .module_deploy_v1(SIGNER, ALICE, module)
        .expect("Deploy valid module");

    let parameter = my_contract::InitParameter { initial_value: 0 };
    // Initialize the contract.
    let init = chain
        .contract_init(
            SIGNER,
            ALICE,
            Energy::from(10_000),
            InitContractPayload {
                amount: Amount::zero(),
                mod_ref: deployment.module_reference,
                init_name: OwnedContractName::new_unchecked("init_my_contract".to_string()),
                param: OwnedParameter::from_serial(&parameter).expect("Serialize 'InitParameter'"),
            },
        )
        .expect("Initializing contract");

    (chain, init)
}
