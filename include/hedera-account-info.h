#pragma once

 #include <stdint.h>
 #include <stdbool.h>
 #include "hedera-id.h"
 #include "hedera-crypto.h"
 #include "hedera-timestamp.h"
 #include "hedera-duration.h"
 #include "hedera-array.h"

 #ifdef __cplusplus
 extern "C" {
 #endif

 typedef struct {
     HederaAccountId account_id;

     char* contract_account_id;
     bool deleted;
     HederaAccountId proxy_account_id;

     int64_t proxy_received;

     HederaPublicKey key;

     uint64_t balance;

     uint64_t generate_send_record_threshold;
     uint64_t generate_receive_record_threshold;

     bool receiver_signature_required;

     HederaTimestamp expiration_time;
     HederaDuration auto_renew_period;

     HederaArray claims;
 } HederaAccountInfo;

 #ifdef __cplusplus
 }
 #endif
