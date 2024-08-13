// use ethers::prelude::*;
// use ethers::providers::{Provider, Http};
// use ethers::contract::Contract;
// use ethers::types::Address;
// use std::sync::Arc;
// use ethers::abi::Abi;
// use std::fs;
// use tokio::sync::RwLock;
// use tokio::task;
// use futures::executor::block_on;
//
// // 已经废弃这个用例
// // ethers = "2.0.14"
// // futures = "0.3.30"
//
// struct ProviderWarp {
//     provider: Arc<Provider<Http>>,
//     contract_address: String,
//     abi_file: String,
//     contract: ContractInstance <Arc<Provider<Http>>, Provider<Http>>,
// }
//
// static PROVIDER_HTTP: tokio::sync::OnceCell<Arc<tokio::sync::RwLock<ProviderWarp>>> = tokio::sync::OnceCell::const_new();
//
// pub async fn initialize_provider_http(url: String, abi_path: String, contract_address_: String)  -> Result<(), Box<dyn std::error::Error>>{
//     let provider = Provider::<Http>::try_from(url.clone())?;
//     let provider = Arc::new(provider);
//
//     let contract_address: Address = contract_address_.clone().parse()?;
//     let abi_json = fs::read(abi_path.clone())?;
//     let abi: Abi = serde_json::from_slice(&abi_json)?;
//
//     let contract    = Contract::new(contract_address, abi, provider.clone());
//
//     let _ = PROVIDER_HTTP.set(Arc::new(RwLock::new(ProviderWarp{
//         provider: provider.clone(),
//         contract_address: contract_address_,
//         abi_file: abi_path,
//         contract: contract.clone(),
//     })));
//
//     Ok(())
// }
// pub async fn get_address(ticket: String) -> Result<String, Box<dyn std::error::Error>> {
//     // let c = PROVIDER_HTTP.get().unwrap().read().await;
//     // ethers 中 Provider 存在一个兼容性问题，它使用了  use std::sync::Mutex
//     // let addr: Address = c.contract.method::<_, Address>("getAddress", ticket)?.call().await?;
//
//     let addr = task::spawn_blocking(move || {
//         // 使用单一 block_on 来执行所有需要同步的异步调用
//         let c = block_on(PROVIDER_HTTP.get().unwrap().read());
//         let address = block_on(c.contract.method::<_, Address>("getAddress", ticket)?.call());
//         address
//     }).await??;
//
//     let a = format!("{:?}", addr);
//     if "0x0000000000000000000000000000000000000000" == a {
//         return Ok("".to_string());
//     }
//     Ok(a)
// }