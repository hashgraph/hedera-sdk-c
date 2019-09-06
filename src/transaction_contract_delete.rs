use hedera::transaction::TransactionContractDelete;
use hedera::ContractId;
use hedera::AccountId;

def_tx_new!(
    TransactionContractDelete: hedera_transaction__contract_delete__new(ContractId)
);

def_tx_method!(
    TransactionContractDelete: hedera_transaction__contract_delete__set_obtainer_account(AccountId):
        obtainer_account
);
