//
//  AGQRResult.m
//  Airgap
//
//  QR processing result
//

#import "AGQRResult.h"

@implementation AGQRResult

- (instancetype)initWithChunkNumber:(NSUInteger)chunkNumber totalChunks:(NSUInteger)totalChunks {
    if (self = [super init]) {
        _chunkNumber = chunkNumber;
        _totalChunks = totalChunks;
    }
    return self;
}

@end
