#pragma once

#include <stdint.h>
#include "hedera-id.h"
#include "hedera-query.h"
#include "hedera-array.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaQuery* hedera_query__file_get_contents__new(
    HederaClient*,
    HederaFileId file_id
);

extern HederaError hedera_query__file_get_contents__get(
    HederaQuery*, 
    HederaArray**
);

#ifdef __cplusplus
}
#endif