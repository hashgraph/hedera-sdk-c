// TODO: Finish this wrapper
use crate::array::CArray;
use hedera::{ContractId};
use hedera::function_result::ContractFunctionResult;
use hedera::function_result::ContractLogInfo;

#[repr(C)]
pub struct CContractLogInfo {
    pub bloom: CArray<u8>,
    pub topic: CArray<CArray<u8>>,
    pub data: CArray<u8>,
}

#[repr(C)]
pub struct CContractFunctionResult {
    pub contract_id: ContractId,
    pub contract_call_result: CArray<u8>,
    pub error_message: *const libc::c_char,
    pub bloom: CArray<u8>,
    pub gas_used: u64,
    pub log_info: CArray<CContractLogInfo>,
}

impl From<ContractFunctionResult> for CContractFunctionResult {
    fn from(result: ContractFunctionResult) -> Self {
        let error_message = mbox::MString::from_str(&result.error_message)
            .into_mbox_with_sentinel()
            .into_raw() as _;

        Self {
            contract_id: result.contract_id,
            contract_call_result: result.contract_call_result.into(),
            error_message,
            bloom: result.bloom.into(),
            gas_used: result.gas_used.into(),
            log_info: result
                .log_info
                .into_iter()
                .map(Into::into)
                .collect::<Vec<CContractLogInfo>>()
                .into(),
        }
    }
}

impl From<ContractLogInfo> for CContractLogInfo {
    fn from(result: ContractLogInfo) -> Self {
        Self {
            bloom: result.bloom.into(),

            topic: result
                .topic
                .into_iter()
                .map(Into::into)
                .collect::<Vec<CArray<u8>>>()
                .into(),

            data: result.data.into(),
        }
    }
}
