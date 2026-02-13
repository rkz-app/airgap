//
//  AGEncoder.m
//  Airgap
//
//  Objective-C wrapper for Airgap encoder
//

#import "AGEncoder.h"
#import "airgap.h"

static NSString *const AGEncoderErrorDomain = @"app.rkz.airgap.encoder";

@implementation AGEncoder {
    struct AirgapEncoder *_encoder;
}

- (nullable instancetype)initWithData:(NSData *)data
                            chunkSize:(NSUInteger)chunkSize
                                error:(NSError **)error {
    if (self = [super init]) {
        if (!data || data.length == 0) {
            if (error) {
                *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                            code:AIRGAP_ERR_NULL_POINTER
                                        userInfo:@{NSLocalizedDescriptionKey: @"Data cannot be nil or empty"}];
            }
            return nil;
        }

        if (chunkSize < MIN_CHUNK_SIZE || chunkSize > MAX_CHUNK_SIZE) {
            if (error) {
                NSString *message = [NSString stringWithFormat:@"Chunk size must be between %d and %d", MIN_CHUNK_SIZE, MAX_CHUNK_SIZE];
                *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                            code:AIRGAP_ERR_CHUNK_SIZE_TOO_LARGE
                                        userInfo:@{NSLocalizedDescriptionKey: message}];
            }
            return nil;
        }

        _encoder = airgap_encoder_new(data.bytes, data.length, chunkSize);

        if (!_encoder) {
            if (error) {
                *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                            code:AIRGAP_UNKNOWN_ERR
                                        userInfo:@{NSLocalizedDescriptionKey: @"Failed to create encoder"}];
            }
            return nil;
        }
    }
    return self;
}

- (void)dealloc {
    if (_encoder) {
        airgap_encoder_free(_encoder);
        _encoder = NULL;
    }
}

- (NSUInteger)chunkCount {
    if (!_encoder) return 0;
    return airgap_encoder_chunk_count(_encoder);
}

- (uint32_t)sessionId {
    if (!_encoder) return 0;
    return airgap_encoder_session_id(_encoder);
}

- (nullable NSData *)generatePNGAtIndex:(NSUInteger)index error:(NSError **)error {
    if (!_encoder) {
        if (error) {
            *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                        code:AIRGAP_ERR_NULL_POINTER
                                    userInfo:@{NSLocalizedDescriptionKey: @"Encoder is not initialized"}];
        }
        return nil;
    }

    struct ByteArray result = {NULL, 0};
    intptr_t status = airgap_encoder_generate_png(_encoder, index, &result);

    if (status != AIRGAP_OK) {
        if (error) {
            NSString *message = [NSString stringWithFormat:@"Failed to generate PNG at index %lu: error code %ld", (unsigned long)index, (long)status];
            *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                        code:status
                                    userInfo:@{NSLocalizedDescriptionKey: message}];
        }
        return nil;
    }

    if (!result.data || result.len == 0) {
        if (error) {
            *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                        code:AIRGAP_UNKNOWN_ERR
                                    userInfo:@{NSLocalizedDescriptionKey: @"Generated empty PNG data"}];
        }
        return nil;
    }

    NSData *pngData = [NSData dataWithBytes:result.data length:result.len];
    airgap_byte_array_free(result);

    return pngData;
}

@end