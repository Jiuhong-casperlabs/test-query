#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use casper_engine_test_support::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
        DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG, DEFAULT_GENESIS_CONFIG_HASH,
        DEFAULT_PAYMENT,
    };
    use casper_execution_engine::core::engine_state::{
        run_genesis_request::RunGenesisRequest, GenesisAccount,
    };
    use casper_types::{
        account::AccountHash, runtime_args, ContractHash, Key, Motes, PublicKey, RuntimeArgs,
        SecretKey, U512,
    };

    const MY_ACCOUNT: [u8; 32] = [7u8; 32];
    const CONTRACT_WASM: &str = "contract.wasm";

    #[test]
    fn should_query() {
        // Create keypair.
        let secret_key = SecretKey::ed25519_from_bytes(MY_ACCOUNT).unwrap();
        let public_key = PublicKey::from(&secret_key);

        // Create an AccountHash from a public key.
        let account_addr = AccountHash::from(&public_key);
        // Create a GenesisAccount.
        let account = GenesisAccount::account(
            public_key,
            Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
            None,
        );

        let mut genesis_config = DEFAULT_GENESIS_CONFIG.clone();
        genesis_config.ee_config_mut().push_account(account);

        let run_genesis_request = RunGenesisRequest::new(
            *DEFAULT_GENESIS_CONFIG_HASH,
            genesis_config.protocol_version(),
            genesis_config.take_ee_config(),
        );
        // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
        // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
        // absolute paths.
        let session_code = PathBuf::from(CONTRACT_WASM);
        let session_args = runtime_args! {};

        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            })
            .with_session_code(session_code, session_args)
            .with_authorization_keys(&[account_addr])
            .with_address(account_addr)
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&run_genesis_request).commit();

        // deploy the contract.
        builder.exec(execute_request).commit().expect_success();

        let result = builder
            .query(None, Key::Account(account_addr), &[])
            .expect("should be stored value1.");
        println!("result is\n {:?}", result);
        println!("========================");

        let a = builder
            .get_account(account_addr)
            .expect("should be account")
            .named_keys()
            .clone();
        println!("named keys of account are\n {:?}", a);
        println!("========================");

        let contracthash = builder
            .get_account(account_addr)
            .expect("should be account")
            .named_keys()
            .get("mycontracthash")
            .cloned()
            .and_then(Key::into_hash)
            // .map(ContractHash::new)
            .expect("should have hash");

        let contract_hash = Some(contracthash).map(ContractHash::new).unwrap();
        println!("contracthash is {:?}", contract_hash);
        println!("========================");

        let named_keys_contract = builder
            .get_contract(contracthash.into())
            .expect("should be contract")
            .named_keys()
            .clone();
        println!("contract named keys are {:?}", named_keys_contract);
        println!("========================");
        let dic_key = *named_keys_contract
            .get("my_dicname")
            .expect("should have key11");
        println!("my_dicname key is {}", dic_key);
        println!("========================");

        // make assertions
        let result_of_query = builder
            .query(
                None,
                Key::Account(account_addr),
                &["mycontracthash".to_string(), "hello".to_string()], //multiple paths
            )
            .expect("should be stored value2.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<String>()
            .expect("should be string.");

        println!("result_of_query is {}", result_of_query);
        println!("========================");

        let _result_of_query = builder
            .query(None, Key::Hash(contracthash), &["my_dicname".to_string()])
            .expect("should be stored value3.")
            .as_cl_value()
            .expect("should be cl value3.")
            .clone()
            .into_t::<()>()
            .expect("should be unit.");

        let _result_of_query = builder
            .query(None, Key::Account(account_addr), &["my-uref".to_string()])
            .expect("should be stored value3.")
            .as_cl_value()
            .expect("should be cl value3.")
            .clone()
            .into_t::<()>()
            .expect("should be unit.");
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
