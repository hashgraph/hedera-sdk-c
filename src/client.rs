use super::errors::HederaResult;
use failure::{bail, Error};
use hedera::{AccountId, Client, SecretKey};
use libc::{c_void, c_char, c_int};
use std::mem::{transmute, MaybeUninit};
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn hedera_client_new(address: *const c_char, out: *mut *mut Client) -> HederaResult {
    let address = unsafe { CStr::from_ptr(address) }.to_string_lossy();
    let client = try_ffi!(Client::new(address));

    unsafe {
        *out = Box::into_raw(Box::new(client));
    }

    HederaResult::Success
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_set_node(client: *mut Client, node: AccountId) {
    (&mut *client).set_node(node);
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_set_operator(
    client: *mut Client,
    operator: AccountId,
    secret: extern "C" fn(user_data: *const c_void, out: *mut SecretKey) -> c_int,
    user_data: *const c_void,
) {
    let user_data: usize = transmute(user_data);
    (&mut *client).set_operator(operator, move || -> Result<SecretKey, Error> {
        let mut key = MaybeUninit::<SecretKey>::uninit();
        let user_data: *const c_void = transmute(user_data);

        let res = secret(user_data, key.as_mut_ptr());
        if res != 0 {
            bail!("failed to get the operator secret key (via callback given to [hedera_client_set_operator])");
        }

        Ok(key.assume_init())
    });
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_close(client: *mut Client) {
    let _ = Box::from_raw(client);
}
