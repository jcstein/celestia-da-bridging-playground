mod online_provider;
mod hana_oracle_payload;

use anyhow::{anyhow, Context};
use celestia_rpc::{BlobClient, Client as CelestiaClient, HeaderClient, ShareClient};
use celestia_types::nmt::Namespace;
use celestia_types::Commitment;
use dotenv::dotenv;
use std::env;
use std::time::Duration;
use alloy_provider::RootProvider;
use base64::Engine;
use celestia_rpc::blobstream::BlobstreamClient;
use crate::online_provider::OnlineCelestiaProvider;

const CELESTIA_BLOCK_HEIGHT: u64 = 4556941;
const BLOB_COMMITMENT: &str = "WEmpiTzmpKBMgCBG4/4csxjsIxOPi87JK5631bWC8yY=";
const BLOB_HASH: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAGVjbGlwc2U=";
const NAMESPACE: [u8; 7] = [0x65, 0x63, 0x6c, 0x69, 0x70, 0x73, 0x65];
fn decode_commitment(b64: &str) -> Result<Commitment, anyhow::Error> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(b64)?;
    let array: [u8; 32] = bytes.try_into().map_err(|_| anyhow!("Invalid length"))?;
    Ok(Commitment::new(array))
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let blobstream_address = alloy_primitives::address!("0x7Cf3876F681Dbb6EdA8f6FfC45D66B996Df08fAe");

    let block_height = CELESTIA_BLOCK_HEIGHT;
    let namespace = Namespace::new_v0(&NAMESPACE)?;
    let blob_commitment: Commitment = decode_commitment(BLOB_COMMITMENT)?;

    let celestia_full_node_rpc_url = env::var("CELESTIA_FULL_NODE_RPC_URL").with_context(|| "CELESTIA_FULL_NODE_RPC_URL must be set")?;
    let celestia_client = CelestiaClient::new(&celestia_full_node_rpc_url, None).await.with_context(|| "Failed to create Celestia client")?;

    let ethereum_rpc_url = env::var("ETHEREUM_RPC_URL").with_context(|| "ETHEREUM_RPC_URL must be set")?;
    let l1_provider = RootProvider::connect(&ethereum_rpc_url).await?;

    let blob = celestia_client.blob_get(block_height, namespace, blob_commitment).await.with_context(|| "Failed to retrieve blob")?;

    let online_provider = OnlineCelestiaProvider::new(celestia_client, namespace, blobstream_address);

    online_provider.generate_oracle_payload(&l1_provider, block_height, blob).await?;
    //
    // let block_header = celestia_client.header_get_by_height(block_height).await?;
    // println!("Block header: {}", block_header);
    //
    // let data_root = block_header.dah.hash();
    // let eds_row_roots = block_header.dah.row_roots();
    // let eds_size = eds_row_roots.len() as u64;
    // let ods_size = eds_size / 2;
    //
    // let index = blob.index.unwrap();
    // let first_row_index = index.div_ceil(eds_size) - 1;
    // let start_index = index - (first_row_index * ods_size);
    // let end_index = start_index + blob.shares_len() as u64;
    //
    // let share_proof = celestia_client.share_get_range(&block_header, start_index, end_index).await?.proof;
    //
    // let event = celestia_client.get_data_root_tuple_inclusion_proof()
    //
    // let blob = celestia_client.blob_get(block_height, namespace, blob_commitment).await?;
    //
    // let ns_proofs = celestia_client.blob_get_proof(block_height, namespace, blob_commitment).await?;
    // println!("Namespace proofs: {:?}", ns_proofs);
    //
    // let eds = celestia_client.share_get_eds(&block_header).await?;
    //
    // // let x = celestia_client.get_data_root_tuple_root()
    //
    // // for ns_proof in ns_proofs {
    // // }
    //
    Ok(())
}
