// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    block::{BlockInfo, BlockRetriever},
    error::ApiResult,
};
use aptos_types::chain_id::ChainId;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct BlockRetrieverMock {
    chain_id: ChainId,
    info_by_height: BTreeMap<u64, ApiResult<BlockInfo>>,
    block_by_height: BTreeMap<u64, ApiResult<aptos_rest_client::aptos_api_types::BcsBlock>>,
}

impl BlockRetrieverMock {
    pub fn new(chain_id: ChainId) -> Self {
        BlockRetrieverMock {
            chain_id,
            info_by_height: BTreeMap::new(),
            block_by_height: BTreeMap::new(),
        }
    }
}

impl BlockRetriever for BlockRetrieverMock {
    async fn get_block_info_by_height(
        &self,
        height: u64,
        chain_id: ChainId,
    ) -> ApiResult<BlockInfo> {
        assert_eq!(
            chain_id, self.chain_id,
            "Wrong chain ID {} expected {}",
            chain_id, self.chain_id
        );

        if let Ok(info) = self.info_by_height.get(&height) {
            return info;
        } else {
            panic!("Height doesn't have a value {}", height);
        }
    }

    async fn get_block_by_height(
        &self,
        height: u64,
        with_transactions: bool,
    ) -> ApiResult<aptos_rest_client::aptos_api_types::BcsBlock> {
        if let Ok(data) = self.get_block_by_height.get(&height) {
            // Throw away transactions if there are none
            if (!with_transactions) {
                if (data.transactions.is_some()) {
                    let _ = data.transactions.take();
                }
            }

            return data;
        } else {
            panic!("Height doesn't have a value {}", height);
        }
    }
}

fn setup_rosetta() {
    let retriever = ();
}

#[test]
fn test_parse_transactions() {}
