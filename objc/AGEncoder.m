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
 								qrSize: (NSUInteger)qrSize
                                error:(NSError **)error {
    if (self = [super init]) {
        // Call Rust FFI - all validation is done on Rust side
        struct CResult result = airgap_encoder_new(data.bytes, data.length, chunkSize, qrSize);

        if (result.code != AIRGAP_OK) {
            if (error) {
                NSString *message =  [NSString stringWithUTF8String:result.error_message];
                *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                            code:result.code
                                        userInfo:@{NSLocalizedDescriptionKey: message}];
            }
            result_error_message_free(result);
            return nil;
        }

        _encoder = (struct AirgapEncoder *)result.payload;
        result_error_message_free(result);
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

- (nullable NSString *)getQRStringAtIndex:(NSUInteger)index error:(NSError **)error {
    if (!_encoder) {
        if (error) {
            *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                        code:-1
                                    userInfo:@{NSLocalizedDescriptionKey: @"Encoder is not initialized"}];
        }
        return nil;
    }

    struct CResult result = airgap_encoder_get_qr_string(_encoder, index);

    if (result.code != AIRGAP_OK) {
        if (error) {
            NSString *message = [NSString stringWithUTF8String:result.error_message];
            *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                        code:result.code
                                    userInfo:@{NSLocalizedDescriptionKey: message}];
        }
        result_error_message_free(result);
        return nil;
    }

    if (!result.payload) {
        if (error) {
            *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                        code:-1
                                    userInfo:@{NSLocalizedDescriptionKey: @"Generated empty QR string"}];
        }
        result_error_message_free(result);
        return nil;
    }

    // Extract ByteArray (which contains null-terminated C string) and convert to NSString
    struct ByteArray *byteArray = (struct ByteArray *)result.payload;
    NSString *qrString = [NSString stringWithUTF8String:(const char *)byteArray->data];

    // Free resources
    airgap_byte_array_free(*byteArray);
    free((void *)result.payload);
    result_error_message_free(result);

    return qrString;
}

- (nullable NSData *)generatePNGAtIndex:(NSUInteger)index error:(NSError **)error {
    if (!_encoder) {
        if (error) {
            *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                        code:-1
                                    userInfo:@{NSLocalizedDescriptionKey: @"Encoder is not initialized"}];
        }
        return nil;
    }

    struct CResult result = airgap_encoder_generate_png(_encoder, index);

    if (result.code != AIRGAP_OK) {
        if (error) {
            NSString *message =  [NSString stringWithUTF8String:result.error_message];
            *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                        code:result.code
                                    userInfo:@{NSLocalizedDescriptionKey: message}];
        }
        result_error_message_free(result);
        return nil;
    }

    if (!result.payload) {
        if (error) {
            *error = [NSError errorWithDomain:AGEncoderErrorDomain
                                        code:-1
                                    userInfo:@{NSLocalizedDescriptionKey: @"Generated empty PNG data"}];
        }
        result_error_message_free(result);
        return nil;
    }

    // Extract ByteArray from payload and convert to NSData
    struct ByteArray *byteArray = (struct ByteArray *)result.payload;
    NSData *pngData = [NSData dataWithBytes:byteArray->data length:byteArray->len];

    // Free resources
    airgap_byte_array_free(*byteArray);
    result_error_message_free(result);

    return pngData;
}

@end