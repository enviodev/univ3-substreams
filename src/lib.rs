mod abi;
mod pb;
extern crate lazy_static;

use pb::example::{Swap, Swaps};

use substreams::hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

const TRACKED_CONTRACT: [u8; 20] = hex!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_swap(block: eth::Block) -> Result<Swaps, substreams::errors::Error> {
    println!("map_swap function called");
    substreams::log::info!("map_swap function called");
    Ok(Swaps {
        swaps: block
            .events::<abi::SwapContract::events::Swap>(&[&TRACKED_CONTRACT])
            .map(|(swap, _log)| {
                substreams::log::info!("Swap {:?}", swap);
                substreams::log::info!("id {:?}", format!("0x{}", hex::encode(block.hash.clone())));
                substreams::log::info!(
                    "sender {:?}",
                    format!("0x{}", hex::encode(swap.sender.clone()))
                );
                substreams::log::info!(
                    "recipient {:?}",
                    format!("0x{}", hex::encode(swap.recipient.clone()))
                );

                let amount0_string = Into::<String>::into(swap.amount0.clone());
                substreams::log::info!("amount0 {:?}", amount0_string);

                let amount1_string = Into::<String>::into(swap.amount1.clone());
                substreams::log::info!("amount1 {:?}", amount1_string);

                let sqrt_price_x96_string = Into::<String>::into(swap.sqrt_price_x96.clone());
                substreams::log::info!("sqrt_price_x96 {:?}", sqrt_price_x96_string);

                let liquidity_string = Into::<String>::into(swap.liquidity.clone());
                substreams::log::info!("liquidity {:?}", liquidity_string);

                let tick_string = Into::<String>::into(swap.tick.clone());
                substreams::log::info!("tick {:?}", tick_string);

                substreams::log::info!("block_number {:?}", block.number);
                substreams::log::info!(
                    "Block hash {:?}",
                    format!("0x{}", hex::encode(block.hash.clone()))
                );
                substreams::log::info!("Swap event seen");
                let swap_instance = Swap {
                    id: format!("0x{}", hex::encode(block.hash.clone())),
                    sender: format!("0x{}", hex::encode(swap.sender.clone())),
                    recipient: format!("0x{}", hex::encode(swap.recipient.clone())),
                    amount0: amount0_string,
                    amount1: amount1_string,
                    sqrt_price_x96: sqrt_price_x96_string,
                    liquidity: liquidity_string,
                    tick: tick_string,
                    block_number: block.number as i64,
                    block_timestamp: block.number as i64,
                    // block_timestamp: block.header.unwrap().timestamp.unwrap() as i64,
                    transaction_hash: format!("0x{}", hex::encode(block.hash.clone())),
                    ordinal: block.number,
                };

                substreams::log::info!("Swap event parsed");
                swap_instance
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn graph_out(swaps: Swaps) -> Result<EntityChanges, substreams::errors::Error> {
    // println!("graph_out function called");
    // substreams::log::info!("graph_out function called");
    // substreams::log::info!("swaps: {:?}", swaps);
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
