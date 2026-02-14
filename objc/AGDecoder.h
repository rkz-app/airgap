//
//  AGDecoder.h
//  Airgap
//
//  Objective-C wrapper for Airgap decoder
//

#import <Foundation/Foundation.h>
#import "AGQRResult.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * AGDecoder decodes QR code chunks back into the original data
 */
@interface AGDecoder : NSObject

/**
 * Creates a new decoder instance
 */
- (instancetype)init;

/**
 * Whether all chunks have been received and the data is complete
 */
@property (nonatomic, readonly, getter=isComplete) BOOL complete;

/**
 * Total number of chunks expected (0 if not yet known)
 */
@property (nonatomic, readonly) NSUInteger totalChunks;

/**
 * Number of unique chunks received so far
 */
@property (nonatomic, readonly) NSUInteger receivedChunks;

/**
 * Process a QR code string
 *
 * @param qrString The string data from a scanned QR code
 * @param error Error pointer for processing failures
 * @return AGQRResult with chunk information if successful, nil otherwise
 */
- (nullable AGQRResult *)processQRString:(NSString *)qrString error:(NSError **)error;

/**
 * Get the decoded data once complete
 *
 * @param error Error pointer for retrieval failures
 * @return The decoded data if complete, or nil if not complete or on error
 */
- (nullable NSData *)getDataWithError:(NSError **)error;

@end

NS_ASSUME_NONNULL_END