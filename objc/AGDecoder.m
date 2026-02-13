//
//  AGDecoder.m
//  Airgap
//
//  Objective-C wrapper for Airgap decoder
//

#import "AGDecoder.h"
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
    if (!_decoder) return NO;
    return airgap_decoder_is_complete(_decoder);
}

- (NSUInteger)totalChunks {
    if (!_decoder) return 0;
    return airgap_decoder_get_total(_decoder);
}

- (NSUInteger)receivedChunks {
    if (!_decoder) return 0;
    return airgap_decoder_get_received(_decoder);
}

- (BOOL)processQRString:(NSString *)qrString error:(NSError **)error {
    if (!_decoder) {
        if (error) {
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:AIRGAP_ERR_NULL_POINTER
                                    userInfo:@{NSLocalizedDescriptionKey: @"Decoder is not initialized"}];
        }
        return NO;
    }

    if (!qrString) {
        if (error) {
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:AIRGAP_ERR_NULL_POINTER
                                    userInfo:@{NSLocalizedDescriptionKey: @"QR string cannot be nil"}];
        }
        return NO;
    }

    const char *cString = [qrString UTF8String];
    intptr_t status = airgap_decoder_process_qr(_decoder, cString);

    if (status != AIRGAP_OK) {
        if (error) {
            NSString *message = [self errorMessageForCode:status];
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:status
                                    userInfo:@{NSLocalizedDescriptionKey: message}];
        }
        return NO;
    }

    return YES;
}

- (nullable NSData *)getDataWithError:(NSError **)error {
    if (!_decoder) {
        if (error) {
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:AIRGAP_ERR_NULL_POINTER
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

    struct ByteArray result = {NULL, 0};
    intptr_t status = airgap_decoder_get_data(_decoder, &result);

    if (status != AIRGAP_OK) {
        if (error) {
            NSString *message = [self errorMessageForCode:status];
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:status
                                    userInfo:@{NSLocalizedDescriptionKey: message}];
        }
        return nil;
    }

    if (!result.data || result.len == 0) {
        if (error) {
            *error = [NSError errorWithDomain:AGDecoderErrorDomain
                                        code:AIRGAP_UNKNOWN_ERR
                                    userInfo:@{NSLocalizedDescriptionKey: @"Retrieved empty data"}];
        }
        return nil;
    }

    NSData *data = [NSData dataWithBytes:result.data length:result.len];
    airgap_byte_array_free(result);

    return data;
}

- (NSString *)errorMessageForCode:(intptr_t)code {
    switch (code) {
        case AIRGAP_OK:
            return @"Success";
        case AIRGAP_ERR_NULL_POINTER:
            return @"Null pointer error";
        case AIRGAP_ERR_INVALID_MAGIC:
            return @"Invalid magic number";
        case AIRGAP_ERR_UNSUPPORTED_VERSION:
            return @"Unsupported version";
        case AIRGAP_ERR_CRC_MISMATCH:
            return @"CRC checksum mismatch";
        case AIRGAP_ERR_SESSION_MISMATCH:
            return @"Session ID mismatch";
        case AIRGAP_ERR_METADATA_MISMATCH:
            return @"Metadata mismatch";
        case AIRGAP_ERR_CHUNK_OUT_OF_BOUNDS:
            return @"Chunk index out of bounds";
        case AIRGAP_ERR_TOO_MANY_CHUNKS:
            return @"Too many chunks";
        case AIRGAP_ERR_CHUNK_SIZE_TOO_LARGE:
            return @"Chunk size too large";
        case AIRGAP_ERR_CHUNK_SIZE_TOO_SMALL:
            return @"Chunk size too small";
        case AIRGAP_ERR_MISSING_CHUNK:
            return @"Missing chunk";
        case AIRGAP_ERR_ENCODING:
            return @"Encoding error";
        default:
            return [NSString stringWithFormat:@"Unknown error: %ld", (long)code];
    }
}

@end