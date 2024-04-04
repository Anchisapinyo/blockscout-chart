use super::types;
use crate::{Client, Result};

pub async fn get(
    client: &Client,
    address_hash: ethers_core::types::Address,
) -> Result<types::SmartContract> {
    let path = format!("/api/v2/smart-contracts/0x{}", hex::encode(address_hash));
    client.get_request(client.build_url(&path)).await
}
