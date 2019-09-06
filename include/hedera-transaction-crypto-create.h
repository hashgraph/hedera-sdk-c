#pragma once

#include <stdint.h>
#include <stdbool.h>
#include "hedera-transaction.h"
#include "hedera-id.h"
#include "hedera-duration.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaTransaction* hedera_transaction__crypto_create__new(HederaClient*);

extern void hedera_transaction__crypto_create__set_initial_balance(
    HederaTransaction*, uint64_t balance);

extern void hedera_transaction__crypto_create__set_key(
    HederaTransaction*, HederaPublicKey key);

extern void hedera_transaction__crypto_create__set_proxy_account(
    HederaTransaction*, HederaAccountId id);

extern void hedera_transaction__crypto_create__set_max_receive_proxy_faction(
    HederaTransaction*, int32_t fraction);

extern void hedera_transaction__crypto_create__set_auto_renew_period(
    HederaTransaction*, HederaDuration period);

extern void hedera_transaction__crypto_create__send_record_threshold(
    HederaTransaction*, int64_t threshold);

extern void hedera_transaction__crypto_create__set_receive_record_threshold(
    HederaTransaction*, int64_t threshold);

extern void hedera_transaction__crypto_create__set_receiver_signature_required(
    HederaTransaction*, bool required);

#ifdef __cplusplus
}
#endif
