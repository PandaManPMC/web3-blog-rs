use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    network::{Ethereum, TransactionBuilder},
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    transports::http::{Client, Http},
};
use alloy::transports::http::reqwest;

pub async fn get_address(ticket: String) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://polygon-mainnet.infura.io/v3/d6c49b20a9bf44fabeda87029c7cf51e";
    let contract_address = "0x63A0B0a446800DFf71C184Ef5D4A526F49a67246";
    let abi_p = "E:/b/gitpmc/web3-blog-rs/blog-rs/common/src/tool/web3_blog_abi.json";

    let provider = ProviderBuilder::new().with_recommended_fillers().on_http(reqwest::Url::parse(url).unwrap());

    // Get the contract ABI.
    let path = std::env::current_dir()?.join(abi_p);
    let artifact = std::fs::read(path).expect("Failed to read artifact");
    let json: serde_json::Value = serde_json::from_slice(&artifact)?;

    // Get `abi` from the artifact.
    let abi_value = json.get("abi").expect("Failed to get ABI from artifact");
    let abi = serde_json::from_str(&abi_value.to_string())?;

    let c_a: Address = Address::parse_checksummed(contract_address, None).unwrap();
    let contract: ContractInstance<Http<Client>, _, Ethereum> =
        ContractInstance::new(c_a, provider.clone(), Interface::new(abi));

    // Retrieve the number, which should be 43.
    let address_val = contract.function("getAddress", &[DynSolValue::from(String::from(ticket))])?.call().await?;
    let addr = address_val.first();
    if addr.is_none() {
        return Ok("".to_string());
    }

    let add = addr.unwrap().as_address();
    if add.is_none() {
        return Ok("".to_string());
    }

    let ad = add.unwrap().0;
    let a = format!("{ad}");
    if "0x0000000000000000000000000000000000000000" == a {
        return Ok("".to_string());
    }
    Ok(a)
}