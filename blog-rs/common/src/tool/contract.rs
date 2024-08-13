use std::sync::Arc;
use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    network::{Ethereum},
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    transports::http::{Client, Http},
};
use alloy::json_abi::JsonAbi;
use alloy::providers::fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller};
use alloy::providers::ReqwestProvider;
use alloy::transports::http::reqwest;
use alloy::transports::http::reqwest::Identity;
use tokio::sync::RwLock;

struct ConfigWarp {
    url: String,
    contract_address: String,
    abi_path: String,
    abi: JsonAbi,
}

static PROVIDER_HTTP: tokio::sync::OnceCell<Arc<tokio::sync::RwLock<ConfigWarp>>> = tokio::sync::OnceCell::const_new();

pub async fn initialize_provider_http(url: String, abi_path: String, contract_address: String)  -> Result<(), String>{
    let path = std::env::current_dir().unwrap().join(abi_path.clone());
    let artifact = std::fs::read(path).expect("Failed to read artifact");
    let json: serde_json::Value = serde_json::from_slice(&artifact).unwrap();

    let abi_value = json.get("abi").expect("Failed to get ABI from artifact");
    let abi = serde_json::from_str(&abi_value.to_string()).unwrap();

    let _ = PROVIDER_HTTP.set(Arc::new(RwLock::new(ConfigWarp{
        url,
        contract_address,
        abi_path,
        abi,
    })));

    return Ok(());
}

pub async fn get_address(ticket: String) -> Result<String, String> {
    let conf = PROVIDER_HTTP.get().unwrap().read().await;

    let provider = ProviderBuilder::new().with_recommended_fillers().on_http(reqwest::Url::parse(conf.url.as_str()).unwrap());

    let c_a: Address = Address::parse_checksummed(conf.contract_address.as_str(), None).unwrap();
    let contract: ContractInstance<Http<Client>,  _, Ethereum> =
        ContractInstance::new(c_a, provider.clone(), Interface::new(conf.abi.clone()));

    let address_call_build = contract.function("getAddress", &[DynSolValue::from(String::from(ticket))]);
    if address_call_build.is_err() {
        return Err(address_call_build.err().unwrap().to_string());
    }

    let address_call = address_call_build.unwrap().call().await;
    if address_call.is_err() {
        return Err(address_call.err().unwrap().to_string());
    }

    let address_val = address_call.unwrap();
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