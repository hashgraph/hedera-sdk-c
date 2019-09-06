use hedera::{query::QueryContractCall, ContractId};
use crate::function_result::CContractFunctionResult;

def_query_new!(QueryContractCall: contract_call(ContractId, i64, array_of(u8), i64) -> QueryContractCall);

def_query_exec!(QueryContractCall: contract_call() -> CContractFunctionResult);
