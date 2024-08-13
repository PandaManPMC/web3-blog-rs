// use ethers::prelude::*;
// use ethers::providers::{Provider, Http};
// use ethers::contract::Contract;
// use ethers::types::Address;
// use std::convert::TryFrom;
// use std::sync::Arc;
//
// // // 已经废弃这个用例
// // // ethers = "2.0.14"
// // // futures = "0.3.30"
//
// #[cfg(test)]
// mod contract_test {
//     use ethers::abi::Abi;
//     use crate::tool;
//     use super::*;
//
//     #[tokio::test]
//     async fn get_address() {
//         let p = plier::files::get_current_dir_str();
//         println!("{:?}", p);
//     }
//
//
//     #[tokio::test]
//     async fn test_initialize_provider_http(){
//         let url = "https://polygon-mainnet.infura.io/v3/d6c49b20a9bf44fabeda87029c7cf51e";
//         let contract_address = "0x63A0B0a446800DFf71C184Ef5D4A526F49a67246";
//         let abi_path = "E:/b/gitpmc/web3-blog-rs/blog-rs/common/src/tool/web3_blog_abi.json";
//
//         let r1 = tool::contract_ethers::initialize_provider_http(url.to_string(), abi_path.to_string(), contract_address.to_string()).await;
//
//         if r1.is_err() {
//             println!("{:?}", r1.err());
//             return;
//         }
//         println!("ok");
//         // let ticket = "f466180c238d4541b1f59fea9b7ceed2".to_string();
//         let ticket = "f466180c238d4541b1f59fea9b7ceed20".to_string();
//
//         let addr = tool::contract_ethers::get_address(ticket).await;
//         if addr.is_err() {
//             println!("{:?}", addr.err());
//             return;
//         }
//         let a = addr.unwrap();
//         println!("{:?}", a);
//     }
//
//     #[tokio::test]
//     async fn t1_t() {
//         let r = t1().await;
//         println!("{:?}", r);
//     }
//
//
//
//     async fn t1() -> Result<(), Box<dyn std::error::Error>> {
//         // 1. 创建一个 HTTP 提供者，用于与以太坊节点通信
//         let provider = Provider::<Http>::try_from("https://polygon-mainnet.infura.io/v3/d6c49b20a9bf44fabeda87029c7cf51e")?;
//         let provider = Arc::new(provider);
//
//         // 2. 定义 ERC-20 合约地址
//         let contract_address: Address = "0x63A0B0a446800DFf71C184Ef5D4A526F49a67246".parse()?;
//
//         // 3. 加载合约的 ABI
//         let abi: Abi = serde_json::from_slice(include_bytes!("E:/b/gitpmc/web3-blog-rs/blog-rs/common/src/tool/web3_blog_abi.json"))?;
//
//         // 4. 创建合约实例
//         let contract = Contract::new(contract_address, abi, provider.clone());
//
//         let ticket = "f466180c238d4541b1f59fea9b7ceed2".to_string();
//
//         let addr: Address = contract.method::<_, Address>("getAddress", ticket)?.call().await?;
//
//         // 7. 打印余额
//         println!("Balance: {}", addr);
//
//         Ok(())
//     }
// }