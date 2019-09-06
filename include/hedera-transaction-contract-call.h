#pragma once

#include <stdint.h>
#include "hedera-transaction.h"
#include "hedera-id.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaTransaction* hedera_transaction__contract_call__new(HederaClient*, HederaContractId id);

extern void hedera_transaction__contract_call__set_gas(HederaTransaction*, uint64_t gas);

extern void hedera_transaction__contract_call__set_amount(HederaTransaction*, uint64_t amount);

extern void hedera_transaction__contract_call__set_function_parameters(
    HederaTransaction*, const uint8_t* parameters, size_t parameters_len);

#ifdef __cplusplus
}
#endif
