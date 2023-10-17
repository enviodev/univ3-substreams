mod abi;
mod pb;
extern crate lazy_static;

use pb::example::{Contract, Contracts};
use pb::example::{Swap, Swaps};

use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth;

// Contract of the uniswap v3 swap pool converted into a hex array
const TRACKED_CONTRACT: [u8; 20] = [
    0x88, 0xe6, 0xA0, 0xc2, 0xdD, 0x26, 0xFE, 0xEb, 0x64, 0xF0, 0x39, 0xa2, 0xc4, 0x12, 0x96, 0xFc,
    0xB3, 0xf5, 0x64, 0x0,
];

#[substreams::handlers::map]
fn map_contract(block: eth::v2::Block) -> Result<Contracts, substreams::errors::Error> {
    let contracts = block
        .calls()
        .filter(|view| !view.call.state_reverted)
        .filter(|view| view.call.call_type == eth::v2::CallType::Create as i32)
        .map(|view| Contract {
            address: format!("0x{}", Hex(&view.call.address)),
            block_number: block.number,
            timestamp: block.timestamp_seconds().to_string(),
            ordinal: view.call.begin_ordinal,
        })
        .collect();

    Ok(Contracts { contracts })
}

#[substreams::handlers::map]
fn map_swap(block: eth::v2::Block) -> Result<Swaps, substreams::errors::Error> {
    let swaps: Vec<Swap> = block
        .events::<abi::SwapContract::events::Swap>(&[&TRACKED_CONTRACT])
        .map(|(swap, log)| {
            substreams::log::info!("Swap event seen");
            Swap {
                id: block.hash.get(0).unwrap().to_string(),
                sender: swap.sender.get(0).unwrap().to_string(),
                recipient: swap.recipient.get(0).unwrap().to_string(),
                amount0: swap.amount0.to_i32() as i64,
                amount1: swap.amount1.to_i32() as i64,
                sqrt_price_x96: swap.sqrt_price_x96.to_u64(),
                liquidity: swap.liquidity.to_u64(),
                tick: swap.tick.to_i32() as i64,
                block_number: block.number as i64,
                block_timestamp: block.timestamp_seconds() as i64,
                transaction_hash: block.hash.clone().get(0).unwrap().to_string(),
            }
        })
        .collect();

    Ok(Swaps { swaps })
}

#[substreams::handlers::map]
pub fn graph_out(swaps: Swaps) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    for swap in swaps.swaps.into_iter() {
        tables
            .create_row("Swap", swap.id)
            .set("sender", swap.sender)
            .set("recipient", swap.recipient)
            .set("amount0", swap.amount0)
            .set("amount1", swap.amount1)
            .set("sqrt_price_x96", swap.sqrt_price_x96)
            .set("liquidity", swap.liquidity)
            .set("tick", swap.tick)
            .set("block_number", swap.block_number)
            .set("block_timestamp", swap.block_timestamp)
            .set("transaction_hash", swap.transaction_hash);
    }

    Ok(tables.to_entity_changes())
}
