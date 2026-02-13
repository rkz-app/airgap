#ifndef AIRGAP_H
#define AIRGAP_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define VERSION 1

#define HEADER_SIZE 16

#define MAX_CHUNK_SIZE 1920

#define RECOMMENDED_MAX_CHUNK_SIZE 1100

#define MIN_CHUNK_SIZE 16

#define AIRGAP_OK 0

#define AIRGAP_UNKNOWN_ERR -1

#define AIRGAP_ERR_NULL_POINTER -2

#define AIRGAP_ERR_INVALID_MAGIC -3

#define AIRGAP_ERR_UNSUPPORTED_VERSION -4

#define AIRGAP_ERR_CRC_MISMATCH -5

#define AIRGAP_ERR_SESSION_MISMATCH -6

#define AIRGAP_ERR_METADATA_MISMATCH -7

#define AIRGAP_ERR_CHUNK_OUT_OF_BOUNDS -8

#define AIRGAP_ERR_TOO_MANY_CHUNKS -9

#define AIRGAP_ERR_CHUNK_SIZE_TOO_LARGE -10

#define AIRGAP_ERR_CHUNK_SIZE_TOO_SMALL -11

#define AIRGAP_ERR_MISSING_CHUNK -12

#define AIRGAP_ERR_ENCODING -13

typedef struct AirgapDecoder AirgapDecoder;

typedef struct AirgapEncoder AirgapEncoder;

typedef struct ByteArray {
  /**
   * Pointer to byte data (may be NULL if empty)
   */
  uint8_t *data;
  /**
   * Length of data in bytes
   */
  uintptr_t len;
} ByteArray;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct AirgapEncoder *airgap_encoder_new(const uint8_t *data,
                                         uintptr_t data_len,
                                         uintptr_t chunk_size);

void airgap_encoder_free(struct AirgapEncoder *encoder);

uintptr_t airgap_encoder_chunk_count(const struct AirgapEncoder *encoder);

uint32_t airgap_encoder_session_id(const struct AirgapEncoder *encoder);

intptr_t airgap_encoder_generate_png(const struct AirgapEncoder *encoder,
                                     uintptr_t index,
                                     struct ByteArray *result);

struct AirgapDecoder *airgap_decoder_new(void);

void airgap_decoder_free(struct AirgapDecoder *decoder);

bool airgap_decoder_is_complete(const struct AirgapDecoder *decoder);

uintptr_t airgap_decoder_get_total(const struct AirgapDecoder *decoder);

uintptr_t airgap_decoder_get_received(const struct AirgapDecoder *decoder);

intptr_t airgap_decoder_process_qr(struct AirgapDecoder *decoder, const char *qr_string);

intptr_t airgap_decoder_get_data(const struct AirgapDecoder *decoder, struct ByteArray *result);

void airgap_byte_array_free(struct ByteArray array);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* AIRGAP_H */
