#pragma once

#include <stdint.h>
#include "hedera-id.h"
#include "hedera-query.h"
#include "hedera-function-result.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaQuery* hedera_query__contract_call__new(
        HederaClient*,
        HederaContractId contract,
        uint64_t gas,
        const uint8_t* parameters, size_t parameters_len,
        uint64_t size
);

extern HederaError hedera_query__contract_call__execute(HederaQuery*, HederaContractFunctionResult*);

#ifdef __cplusplus
}
#endif
