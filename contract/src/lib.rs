use near_sdk::{env, near_bindgen, Promise};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const NEAR_TOKEN_DECIMALS: u128 = 24;
const BASE_PRICE: u128 = 100; // 1 NEAR = 100 USD
const LIQUIDATION_THRESHOLD: u128 = 0.6; // When user's margin ratio falls below this threshold, their position is liquidated

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Position {
    pub margin: u128,
    pub size: i128,
    pub entry_price: u128,
    pub stop_loss_price: Option<u128>,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct NearSquared {
    pub index_price: u128,
    pub long_positions: Vec<Position>,
    pub short_positions: Vec<Position>,
    pub total_margin: u128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub near_squared: NearSquared,
}

#[near_bindgen]
impl Contract {
    pub fn long(&mut self, margin: u128) {
        let index_price = self.squeeth.index_price;
        let size = margin / BASE_PRICE;
        let entry_price = index_price;
        let position = Position {
            margin,
            size,
            entry_price,
            stop_loss_price: None,
        };
        self.squeeth.long_positions.push(position);
        self.squeeth.total_margin += margin;
    }

    pub fn short(&mut self, margin: u128) {
        let index_price = self.squeeth.index_price;
        let size = margin / BASE_PRICE;
        let entry_price = index_price;
        let position = Position {
            margin,
            size: -size,
            entry_price,
            stop_loss_price: None,
        };
        self.squeeth.short_positions.push(position);
        self.squeeth.total_margin += margin;
    }

    pub fn close_position(&mut self, index: usize) {
        let position = self.get_position(index);
        let size = position.size;
        let entry_price = position.entry_price;
        let exit_price = self.squeeth.index_price;
        let pnl = if size > 0 {
            (exit_price - entry_price) * size as u128
        } else {
            (entry_price - exit_price) * size.abs() as u128
        };
        let margin = position.margin;
        let net_margin = margin + pnl;
        self.squeeth.total_margin -= margin;
        if size > 0 {
            self.squeeth.long_positions.remove(index);
        } else {
            self.squeeth.short_positions.remove(index);
        }
    }

    pub fn liquidate_position(&mut self, index: usize) {
        let position = self.get_position(index);
        let size = position.size;
        let entry_price = position.entry_price;
        let exit_price = self.squeeth.index_price;
        let pnl = if size > 0 {
            (exit_price - entry_price) * size as u128
        } else {
            (entry_price - exit_price) * size.ab

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_long_position() {
        todo!();
    }
}
