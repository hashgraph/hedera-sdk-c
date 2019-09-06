use hedera::transaction::TransactionContractUpdate;
use hedera::AccountId;
use hedera::ContractId;
use hedera::FileId;
use hedera::PublicKey;

def_tx_new!(TransactionContractUpdate: hedera_transaction__contract_update__new(ContractId));

// todo: expires at

def_tx_method!(
    TransactionContractUpdate: hedera_transaction__contract_update__set_admin_key(PublicKey):
        admin_key
);

def_tx_method!(
    TransactionContractUpdate: hedera_transaction__contract_update__set_proxy_account(AccountId):
        proxy_account
);

// todo: auto renew period

def_tx_method!(
    TransactionContractUpdate: hedera_transaction__contract_update__set_file(FileId): file
);
