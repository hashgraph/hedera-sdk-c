#pragma once

#include <stdint.h>
#include "hedera-transaction.h"
#include "hedera-id.h"
#include "hedera-duration.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaTransaction* hedera_transaction__contract_delete__new(HederaClient*, HederaContractId id);

extern void hedera_transaction__contract_delete__set_obtainer_account(HederaTransaction*, HederaAccountId id);

#ifdef __cplusplus
}
#endif