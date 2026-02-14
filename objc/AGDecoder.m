//
//  AGDecoder.m
//  Airgap
//
//  Objective-C wrapper for Airgap decoder
//

#import "AGDecoder.h"
#import "AGQRResult.h"
#import "airgap.h"

static NSString *const AGDecoderErrorDomain = @"app.rkz.airgap.decoder";

@implementation AGDecoder {
    struct AirgapDecoder *_decoder;
}

- (instancetype)init {
    if (self = [super init]) {
        _decoder = airgap_decoder_new();
        if (!_decoder) {
            return nil;
        }
    }
    return self;
}

- (void)dealloc {
    if (_decoder) {
        airgap_decoder_free(_decoder);
        _decoder = NULL;
    }
}

- (BOOL)isComplete {
    return airgap_decoder_is_complete(_decoder);
}

- (NSUInteger)totalChunks {
    return airgap_decoder_get_total(_decoder);
}

- (NSUInteger)receivedChunks {
    return airgap_decoder_get_received(_decoder);
}

- (NSInteger)sessionId {
    if (!_decoder) return -1;
    return airgap_decoder_get_session_id(_decoder);
}

- (void)reset {
    if (_decoder) {
        airgap_decoder_reset(_decoder);
    }
}

- (nullable AGQRResult *)processQRString:(NSString *)qrString error:(NSError **)error {
    if (!_decoder) {
        if (error) {
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:-1
                                    userInfo:@{NSLocalizedDescriptionKey: @"Decoder is not initialized"}];
        }
        return nil;
    }

    if (!qrString) {
        if (error) {
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:-1
                                    userInfo:@{NSLocalizedDescriptionKey: @"QR string cannot be nil"}];
        }
        return nil;
    }

    const char *cString = [qrString UTF8String];
    struct CResult result = airgap_decoder_process_qr(_decoder, cString);

    if (result.code != AIRGAP_OK) {
        if (error) {
            NSString *message =  [NSString stringWithUTF8String:result.error_message];
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:result.code
                                    userInfo:@{NSLocalizedDescriptionKey: message}];
        }
        result_error_message_free(result);
        return nil;
    }

    // Extract QRResult from payload
    AGQRResult *qrResult = nil;
    if (result.payload) {
        struct QRResult *qr = (struct QRResult *)result.payload;
        qrResult = [[AGQRResult alloc] initWithChunkNumber:qr->chunk_number totalChunks:qr->total_chunk_count];
        free((void *)result.payload);
    }

    result_error_message_free(result);
    return qrResult;
}

- (nullable NSData *)getDataWithError:(NSError **)error {
    if (!_decoder) {
        if (error) {
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:-1
                                    userInfo:@{NSLocalizedDescriptionKey: @"Decoder is not initialized"}];
        }
        return nil;
    }

    if (![self isComplete]) {
        if (error) {
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:AIRGAP_ERR_MISSING_CHUNK
                                    userInfo:@{NSLocalizedDescriptionKey: @"Decoding is not complete yet"}];
        }
        return nil;
    }

    struct CResult result = airgap_decoder_get_data(_decoder);

    if (result.code != AIRGAP_OK) {
        if (error) {
            NSString *message =  [NSString stringWithUTF8String:result.error_message];
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:result.code
                                    userInfo:@{NSLocalizedDescriptionKey: message}];
        }
        result_error_message_free(result);
        return nil;
    }

    if (!result.payload) {
        if (error) {
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:-1
                                    userInfo:@{NSLocalizedDescriptionKey: @"Retrieved empty data"}];
        }
        result_error_message_free(result);
        return nil;
    }

    // Extract ByteArray from payload and convert to NSData
    struct ByteArray *byteArray = (struct ByteArray *)result.payload;
    NSData *data = [NSData dataWithBytes:byteArray->data length:byteArray->len];

    // Free resources
    airgap_byte_array_free(*byteArray);
    result_error_message_free(result);

    return data;
}

@end