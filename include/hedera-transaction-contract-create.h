#pragma once

#include <stdint.h>
#include "hedera-transaction.h"
#include "hedera-id.h"
#include "hedera-duration.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaTransaction* hedera_transaction__contract_create__new(HederaClient*);

extern void hedera_transaction__contract_create__set_file(HederaTransaction*, HederaFileId id);

extern void hedera_transaction__contract_create__set_gas(HederaTransaction*, uint64_t gas);

extern void hedera_transaction__contract_create__set_admin_key(
    HederaTransaction*, HederaPublicKey key);

extern void hedera_transaction__contract_create__set_initial_balance(
    HederaTransaction*, uint64_t balance);

extern void hedera_transaction__contract_create__set_proxy_account(
    HederaTransaction*, HederaAccountId id);

extern void hedera_transaction__contract_create__set_auto_renew_period(
    HederaTransaction*, HederaDuration period);

extern void hedera_transaction__contract_create__set_constructor_parameters(
    HederaTransaction*, const uint8_t* parameters, size_t parameters_t);

#ifdef __cplusplus
}
#endif
