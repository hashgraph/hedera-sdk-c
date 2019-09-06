#pragma once

#include <stdint.h>
#include "hedera-array.h"
#include "hedera-id.h"
#include "hedera-crypto.h"
#include "hedera-timestamp.h"
#include "hedera-duration.h"
#include "hedera-transaction-receipt.h"
#include "hedera-function-result.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    HederaAccountId account_id;
    int64_t amount;
} HederaTransfer;

typedef struct {
    HederaTransactionReceipt receipt;
    HederaArray transaction_hash;
    HederaTimestamp consensus_timestamp;
    char* memo;
    uint64_t transaction_fee;
    HederaContractFunctionResult contract_function_result;
    HederaArray transfers;
} HederaTransactionRecord;

#ifdef __cplusplus
}
#endif
