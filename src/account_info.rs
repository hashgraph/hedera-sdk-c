use crate::{array::CArray, claim::CClaim, duration::CDuration, timestamp::CTimestamp};
use hedera::Claim;
use hedera::{AccountId, AccountInfo, PublicKey};
use mbox::MString;
use std::convert::TryFrom;

#[repr(C)]
#[derive(Debug)]
pub struct CAccountInfo {
    pub account_id: AccountId,
    pub contract_account_id: *const libc::c_char,
    pub deleted: bool,
    pub proxy_account_id: Option<Box<AccountId>>,
    pub proxy_received: i64,
    pub key: PublicKey,
    pub balance: u64,
    pub generate_send_record_threshold: u64,
    pub generate_receive_record_threshold: u64,
    pub receiver_signature_required: bool,
    pub expiration_time: CTimestamp,
    pub auto_renew_period: CDuration,
    pub claims: CArray<CClaim>,
}

impl Drop for CAccountInfo {
    fn drop(&mut self) {
        if !self.contract_account_id.is_null() {
            unsafe {
                libc::free(self.contract_account_id as _);
            }
        }
    }
}

impl From<AccountInfo> for CAccountInfo {
    fn from(account_info: AccountInfo) -> Self {
        let contract_account_id = MString::from_str(&account_info.contract_account_id)
            .into_mbox_with_sentinel()
            .into_raw() as _;

        let claims = account_info
            .claims
            .into_iter()
            .map(Into::into)
            .collect::<Vec<CClaim>>()
            .into();

        CAccountInfo {
            account_id: account_info.account_id,
            contract_account_id,
            deleted: account_info.deleted,
            proxy_account_id: account_info.proxy_account_id.map(|t| Box::new(t.into())),
            proxy_received: account_info.proxy_received,
            key: account_info.key,
            balance: account_info.balance,
            generate_send_record_threshold: account_info.generate_send_record_threshold,
            generate_receive_record_threshold: account_info.generate_receive_record_threshold,
            receiver_signature_required: account_info.receiver_signature_required,
            expiration_time: account_info.expiration_time.into(),
            auto_renew_period: account_info.auto_renew_period.into(),
            claims,
        }
    }
}

impl TryFrom<CAccountInfo> for AccountInfo {
    type Error = failure::Error;

    fn try_from(c_account_info: CAccountInfo) -> Result<Self, Self::Error> {
        let contract_account_id =
            unsafe { std::ffi::CStr::from_ptr(c_account_info.contract_account_id) }
                .to_str()?
                .into();

        let proxy_account_id = match c_account_info.proxy_account_id.clone() {
            Some(proxy_account_id) => Some(*proxy_account_id),
            None => None,
        };

        let claims = unsafe {
            Vec::from_raw_parts(
                c_account_info.claims.ptr,
                c_account_info.claims.len,
                c_account_info.claims.len,
            )
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Claim>>()
            .into()
        };

        Ok(AccountInfo {
            account_id: c_account_info.account_id,
            contract_account_id,
            deleted: c_account_info.deleted,
            proxy_account_id,
            proxy_received: c_account_info.proxy_received,
            key: c_account_info.key.clone(),
            balance: c_account_info.balance,
            generate_send_record_threshold: c_account_info.generate_send_record_threshold,
            generate_receive_record_threshold: c_account_info.generate_receive_record_threshold,
            receiver_signature_required: c_account_info.receiver_signature_required,
            expiration_time: c_account_info.expiration_time.into(),
            auto_renew_period: c_account_info.auto_renew_period.into(),
            claims,
        })
    }
}
