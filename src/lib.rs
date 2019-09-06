#![feature(untagged_unions)]

#[macro_use]
mod macros;

#[macro_use]
mod array;

mod alloc;
mod claim;
mod client;
mod crypto;
mod duration;
mod entity;
mod errors;
mod id;
mod timestamp;
mod transaction_id;
mod transaction_record;
mod function_result;

mod account_info;
mod contract_info;
mod file_info;

mod query_contract_get_bytecode;
mod query_contract_get_info;
mod query_contract_get_records;
mod query_contract_call;

mod query_crypto_get_account_balance;
mod query_crypto_get_account_records;
mod query_crypto_get_claim;
mod query_crypto_get_info;

mod query_file_get_contents;
mod query_file_get_info;

mod query_get_by_key;

mod query_transaction_get_receipt;
mod query_transaction_get_record;

mod transaction;

mod transaction_contract_call;
mod transaction_contract_create;
mod transaction_contract_delete;
mod transaction_contract_update;

mod transaction_crypto_add_claim;
mod transaction_crypto_create;
mod transaction_crypto_delete;
mod transaction_crypto_transfer;
mod transaction_crypto_update;

mod transaction_file_append;
mod transaction_file_create;
mod transaction_file_delete;
mod transaction_file_update;
