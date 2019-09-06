#pragma once

#include <stdint.h>
#include "hedera-id.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    HederaContractId contract_id;
    HederaArray bloom;
    HederaArray topic;
    HederaArray data;
} HederaContractLogInfo;

typedef struct {
    HederaContractId contract_id;
    HederaArray contract_call_result;
    char* error_message;
    HederaArray bloom;
    uint64_t gas_used;
    HederaArray log_info;
} HederaContractFunctionResult;

#ifdef __cplusplus
}
#endif