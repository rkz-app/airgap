//
//  AGEncoder.h
//  Airgap
//
//  Objective-C wrapper for Airgap encoder
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

/**
 * AGEncoder encodes data into QR code chunks for air-gapped communication
 */
@interface AGEncoder : NSObject

/**
 * Creates a new encoder with the given data and chunk size
 *
 * @param data The data to encode
 * @param chunkSize Size of each chunk (must be between MIN_CHUNK_SIZE and MAX_CHUNK_SIZE)
 * @param error Error pointer for initialization failures
 * @return A new encoder instance, or nil if initialization fails
 */
- (nullable instancetype)initWithData:(NSData *)data
                            chunkSize:(NSUInteger)chunkSize
                                error:(NSError **)error;

/**
 * The total number of chunks this encoder will generate
 */
@property (nonatomic, readonly) NSUInteger chunkCount;

/**
 * The session ID for this encoding session
 */
@property (nonatomic, readonly) uint32_t sessionId;

/**
 * Gets the QR code string for the chunk at the given index (base45-encoded)
 * This is what would be scanned from a QR code
 *
 * @param index The chunk index (0-based)
 * @param error Error pointer for retrieval failures
 * @return QR code string (base45-encoded), or nil if retrieval fails
 */
- (nullable NSString *)getQRStringAtIndex:(NSUInteger)index error:(NSError **)error;

/**
 * Generates a PNG image for the chunk at the given index
 *
 * @param index The chunk index (0-based)
 * @param error Error pointer for generation failures
 * @return PNG image data, or nil if generation fails
 */
- (nullable NSData *)generatePNGAtIndex:(NSUInteger)index error:(NSError **)error;

@end

NS_ASSUME_NONNULL_END