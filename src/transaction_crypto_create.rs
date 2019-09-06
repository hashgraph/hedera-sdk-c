use crate::duration::CDuration;
use hedera::transaction::TransactionCryptoCreate;
use hedera::AccountId;
use hedera::PublicKey;

def_tx_new!(TransactionCryptoCreate: hedera_transaction__crypto_create__new);

def_tx_method!(TransactionCryptoCreate: hedera_transaction__crypto_create__set_key(PublicKey): key);

def_tx_method!(
    TransactionCryptoCreate: hedera_transaction__crypto_create__set_initial_balance(u64):
        initial_balance
);

def_tx_method!(
    TransactionCryptoCreate: hedera_transaction__crypto_create__set_proxy_account(AccountId):
        proxy_account
);

def_tx_method!(
    TransactionCryptoCreate: hedera_transaction__crypto_create__set_auto_renew_period(CDuration):
        auto_renew_period
);

def_tx_method!(
    TransactionCryptoCreate: hedera_transaction__crypto_create__set_send_record_threshold(i64):
        send_record_threshold
);

def_tx_method!(
    TransactionCryptoCreate: hedera_transaction__crypto_create__set_receive_record_threshold(i64):
        receive_record_threshold
);

def_tx_method!(
    TransactionCryptoCreate:
        hedera_transaction__crypto_create__set_receiver_signature_required(bool):
        receiver_signature_required
);

// todo: to proto
