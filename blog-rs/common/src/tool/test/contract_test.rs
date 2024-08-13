use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    network::{Ethereum, TransactionBuilder},
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    transports::http::{Client, Http},
};
use alloy::transports::http::reqwest;


#[cfg(test)]
mod contract_test {
    use crate::tool;
    use super::*;

    #[tokio::test]
    async fn get_address() {
        let p = plier::files::get_current_dir_str();
        println!("{:?}", p);
        // let ticket = "f466180c238d4541b1f59fea9b7ceed2".to_string();
        let ticket = "f466180c238d4541b1f59fea9b7ceed2_fd".to_string();
        let r=tool::contract::get_address(ticket).await;
        println!("{:?}", r);
    }

    #[tokio::test]
    async fn t1_t() {
        let r = t1().await;
        println!("{:?}", r);
    }

    async fn t1() -> Result<(), Box<dyn std::error::Error>> {

        let url = "https://polygon-mainnet.infura.io/v3/d6c49b20a9bf44fabeda87029c7cf51e";
        let contract_address = "0x63A0B0a446800DFf71C184Ef5D4A526F49a67246";
        let ticket = "f466180c238d4541b1f59fea9b7ceed2".to_string();
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
        let addr = address_val.first().unwrap().as_address().unwrap().0;

        println!("Retrieved addr: {addr}");

        Ok(())
    }
}