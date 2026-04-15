#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec, Address
};
use core::option::Option;

#[contracttype]
#[derive(Clone)]
pub struct Item {
    id: u64,
    name: String,
    description: String,
    owner: Address,
    finder: Option<Address>,
    status: u32, // 0=lost, 1=claimed, 2=returned
}

const ITEM_DATA: Symbol = symbol_short!("ITEM");
const COUNTER: Symbol = symbol_short!("COUNT");

#[contract]
pub struct LostFoundContract;

#[contractimpl]
impl LostFoundContract {

    pub fn get_items(env: Env) -> Vec<Item> {
        env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env))
    }

    // ➕ report (user harus auth)
    pub fn report_lost(env: Env, user: Address, name: String, description: String) {
        user.require_auth();

        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));

        let mut id: u64 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        id += 1;
        env.storage().instance().set(&COUNTER, &id);

        let item = Item {
            id,
            name,
            description,
            owner: user,
            finder: Option::None,
            status: 0,
        };

        items.push_back(item);
        env.storage().instance().set(&ITEM_DATA, &items);
    }

    // 🙋 claim
    pub fn claim_item(env: Env, user: Address, id: u64) {
        user.require_auth();

        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let mut item = items.get(i).unwrap();

            if item.id == id && item.status == 0 {
                item.finder = Option::Some(user);
                item.status = 1;

                items.set(i, item);
                env.storage().instance().set(&ITEM_DATA, &items);
                return;
            }
        }
    }

    // 🔒 confirm (owner only)
    pub fn confirm_return(env: Env, user: Address, id: u64) {
        user.require_auth();

        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let mut item = items.get(i).unwrap();

            if item.id == id {

                if item.owner != user {
                    panic!("Not owner");
                }

                item.status = 2;

                items.set(i, item);
                env.storage().instance().set(&ITEM_DATA, &items);
                return;
            }
        }
    }
}  