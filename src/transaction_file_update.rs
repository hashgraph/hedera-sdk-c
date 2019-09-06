use hedera::transaction::TransactionFileUpdate;
use hedera::FileId;
use hedera::PublicKey;

def_tx_new!(TransactionFileUpdate: hedera_transaction__file_update__new(FileId));

// todo: expires at

def_tx_method!(TransactionFileUpdate: hedera_transaction__file_update__set_key(PublicKey): key);

def_tx_method!(
    TransactionFileUpdate: hedera_transaction__file_update__set_contents(array_of(u8)): contents
);
