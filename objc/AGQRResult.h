//
//  AGQRResult.h
//  Airgap
//
//  QR processing result
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

/**
 * Result from processing a QR code chunk
 */
@interface AGQRResult : NSObject


- (instancetype)initWithChunkNumber:(NSUInteger)chunkNumber totalChunks:(NSUInteger)totalChunks;

/**
 * The chunk number that was processed (0-based index)
 */
@property (nonatomic, readonly) NSUInteger chunkNumber;

/**
 * The total number of chunks in this session
 */
@property (nonatomic, readonly) NSUInteger totalChunks;

@end

NS_ASSUME_NONNULL_END
