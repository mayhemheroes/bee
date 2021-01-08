// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//#![warn(missing_docs)]

pub mod error;
pub mod event;
pub mod model;
pub mod storage;

mod merkle_hasher;
mod metadata;
mod white_flag;
mod worker;

use storage::StorageBackend;
use worker::LedgerWorker;

use bee_common_pt2::node::{Node, NodeBuilder};
use bee_tangle::milestone::MilestoneIndex;

pub const IOTA_SUPPLY: u64 = 2_779_530_283_277_761;

pub fn init<N: Node>(index: u32, node_builder: N::Builder) -> N::Builder
where
    N::Backend: StorageBackend,
{
    node_builder.with_worker_cfg::<LedgerWorker>(MilestoneIndex(index))
}
