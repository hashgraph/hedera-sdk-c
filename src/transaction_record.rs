use crate::array::CArray;
use crate::timestamp::CTimestamp;
use hedera::TransactionReceipt;
use hedera::{
    AccountId, TransactionRecord, TransactionRecordBody,
};
use crate::function_result::CContractFunctionResult;

#[repr(C)]
pub struct CTransfer {
    pub account_id: AccountId,
    pub amount: i64,
}

#[repr(C)]
pub struct CTransactionRecord {
    pub receipt: TransactionReceipt,
    pub transaction_hash: CArray<u8>,
    pub consensus_timestamp: Option<Box<CTimestamp>>,
    pub memo: *const libc::c_char,
    pub transaction_fee: u64,
    pub contract_function_result: Option<Box<CContractFunctionResult>>,
    pub transfers: CArray<CTransfer>,
}

impl Drop for CTransactionRecord {
    fn drop(&mut self) {
        if !self.memo.is_null() {
            unsafe {
                libc::free(self.memo as _);
            }
        }

        if let Some(result) = &self.contract_function_result {
            if !result.error_message.is_null() {
                unsafe {
                    libc::free(result.error_message as _);
                }
            }
        }
    }
}

impl From<TransactionRecord> for CTransactionRecord {
    fn from(record: TransactionRecord) -> Self {
        let memo = mbox::MString::from_str(&record.memo)
            .into_mbox_with_sentinel()
            .into_raw() as _;

        let (transfers, contract_function_result) = match record.body {
            TransactionRecordBody::Transfer(transfers) => (
                transfers
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<CTransfer>>()
                    .into(),
                None,
            ),

            TransactionRecordBody::ContractCall(result) => {
                (vec![].into(), Some(Box::new(result.into())))
            }

            TransactionRecordBody::ContractCreate(result) => {
                (vec![].into(), Some(Box::new(result.into())))
            }
        };

        Self {
            receipt: record.receipt.into(),
            transaction_hash: record.transaction_hash.into(),
            consensus_timestamp: record.consensus_timestamp.map(|t| Box::new(t.into())),
            memo,
            transaction_fee: record.transaction_fee.into(),
            contract_function_result,
            transfers,
        }
    }
}

impl From<(AccountId, i64)> for CTransfer {
    fn from(transfer: (AccountId, i64)) -> CTransfer {
        Self {
            account_id: transfer.0,
            amount: transfer.1,
        }
    }
}

vec_to_carray!(TransactionRecord, CTransactionRecord);
