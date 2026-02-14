//
//  AirgapTests.m
//  Airgap
//
//  Roundtrip tests for encoder and decoder
//

#import <Foundation/Foundation.h>
#import "AGEncoder.h"
#import "AGDecoder.h"

#define TEST_ASSERT(condition, message) \
    if (!(condition)) { \
        NSLog(@"❌ FAILED: %s - %@", #condition, message); \
        return NO; \
    } else { \
        NSLog(@"✅ PASSED: %s", #condition); \
    }

#define TEST_ASSERT_EQUAL(actual, expected, message) \
    if ((actual) != (expected)) { \
        NSLog(@"❌ FAILED: Expected %@ but got %@ - %@", @(expected), @(actual), message); \
        return NO; \
    } else { \
        NSLog(@"✅ PASSED: %@ == %@", @(actual), @(expected)); \
    }

BOOL testBasicRoundtrip(void) {
    NSLog(@"\n=== Testing Basic Roundtrip ===");

    // Create test data
    NSString *testString = @"Hello, Airgap!";
    NSData *originalData = [testString dataUsingEncoding:NSUTF8StringEncoding];

    // Create encoder
    NSError *error = nil;
    AGEncoder *encoder = [[AGEncoder alloc] initWithData:originalData chunkSize:100 error:&error];
    TEST_ASSERT(encoder != nil, ([NSString stringWithFormat:@"Failed to create encoder: %@", error]));
    TEST_ASSERT(error == nil, @"Error should be nil");

    // Check chunk count
    NSUInteger chunkCount = encoder.chunkCount;
    NSLog(@"Chunk count: %lu", (unsigned long)chunkCount);
    TEST_ASSERT(chunkCount > 0, @"Should have at least one chunk");

    // Get session ID
    uint32_t sessionId = encoder.sessionId;
    NSLog(@"Session ID: %u", sessionId);
    TEST_ASSERT(sessionId != 0, @"Session ID should not be zero");

    // Create decoder
    AGDecoder *decoder = [[AGDecoder alloc] init];
    TEST_ASSERT(decoder != nil, @"Failed to create decoder");

    // Process all chunks
    for (NSUInteger i = 0; i < chunkCount; i++) {
        // Get QR string (this is what would be scanned from a QR code)
        NSString *qrString = [encoder getQRStringAtIndex:i error:&error];
        TEST_ASSERT(qrString != nil, ([NSString stringWithFormat:@"Failed to get QR string at index %lu: %@", i, error]));
        TEST_ASSERT(qrString.length > 0, @"QR string should not be empty");

        // Process in decoder
        AGQRResult *qrResult = [decoder processQRString:qrString error:&error];
        TEST_ASSERT(qrResult != nil, ([NSString stringWithFormat:@"Failed to process QR at index %lu: %@", i, error]));
        TEST_ASSERT_EQUAL(qrResult.chunkNumber, i, @"Chunk number mismatch");
        TEST_ASSERT_EQUAL(qrResult.totalChunks, chunkCount, @"Total chunks mismatch");

        NSLog(@"Processed chunk %lu/%lu", (unsigned long)qrResult.chunkNumber + 1, (unsigned long)qrResult.totalChunks);
    }

    // Check decoder is complete
    TEST_ASSERT(decoder.isComplete, @"Decoder should be complete");
    TEST_ASSERT_EQUAL(decoder.receivedChunks, chunkCount, @"Received chunks mismatch");
    TEST_ASSERT_EQUAL(decoder.totalChunks, chunkCount, @"Total chunks mismatch");
    TEST_ASSERT_EQUAL((uint32_t)decoder.sessionId, sessionId, @"Session ID mismatch");

    // Get decoded data
    NSData *decodedData = [decoder getDataWithError:&error];
    TEST_ASSERT(decodedData != nil, ([NSString stringWithFormat:@"Failed to get decoded data: %@", error]));
    TEST_ASSERT_EQUAL(decodedData.length, originalData.length, @"Decoded data length mismatch");
    TEST_ASSERT([decodedData isEqualToData:originalData], @"Decoded data does not match original");

    NSString *decodedString = [[NSString alloc] initWithData:decodedData encoding:NSUTF8StringEncoding];
    NSLog(@"Original: %@", testString);
    NSLog(@"Decoded:  %@", decodedString);
    TEST_ASSERT([decodedString isEqualToString:testString], @"Decoded string does not match original");

    return YES;
}

BOOL testLargeDataRoundtrip(void) {
    NSLog(@"\n=== Testing Large Data Roundtrip ===");

    // Create larger test data (5KB)
    NSMutableData *largeData = [NSMutableData dataWithCapacity:5000];
    for (int i = 0; i < 5000; i++) {
        uint8_t byte = i % 256;
        [largeData appendBytes:&byte length:1];
    }

    // Create encoder with smaller chunk size to test multiple chunks
    NSError *error = nil;
    AGEncoder *encoder = [[AGEncoder alloc] initWithData:largeData chunkSize:500 error:&error];
    TEST_ASSERT(encoder != nil, ([NSString stringWithFormat:@"Failed to create encoder: %@", error]));

    NSUInteger chunkCount = encoder.chunkCount;
    NSLog(@"Large data chunk count: %lu", (unsigned long)chunkCount);
    TEST_ASSERT(chunkCount > 5, @"Should have multiple chunks");

    // Create decoder
    AGDecoder *decoder = [[AGDecoder alloc] init];
    TEST_ASSERT(decoder != nil, @"Failed to create decoder");

    // Process all chunks in order
    for (NSUInteger i = 0; i < chunkCount; i++) {
        NSString *qrString = [encoder getQRStringAtIndex:i error:&error];
        TEST_ASSERT(qrString != nil, ([NSString stringWithFormat:@"Failed to get QR string at %lu: %@", i, error]));

        AGQRResult *qrResult = [decoder processQRString:qrString error:&error];
        TEST_ASSERT(qrResult != nil, ([NSString stringWithFormat:@"Failed to process chunk %lu: %@", i, error]));
    }

    TEST_ASSERT(decoder.isComplete, @"Decoder should be complete");

    // Get and verify decoded data
    NSData *decodedData = [decoder getDataWithError:&error];
    TEST_ASSERT(decodedData != nil, @"Failed to get decoded data");
    TEST_ASSERT([decodedData isEqualToData:largeData], @"Decoded data does not match original");

    NSLog(@"Successfully roundtripped %lu bytes in %lu chunks",
          (unsigned long)largeData.length, (unsigned long)chunkCount);

    return YES;
}

BOOL testOutOfOrderChunks(void) {
    NSLog(@"\n=== Testing Out of Order Chunks ===");

    // Create test data
    NSString *testString = @"Out of order test data";
    NSData *originalData = [testString dataUsingEncoding:NSUTF8StringEncoding];

    NSError *error = nil;
    AGEncoder *encoder = [[AGEncoder alloc] initWithData:originalData chunkSize:50 error:&error];
    TEST_ASSERT(encoder != nil, @"Failed to create encoder");

    NSUInteger chunkCount = encoder.chunkCount;
    AGDecoder *decoder = [[AGDecoder alloc] init];

    // Collect all QR strings first
    NSMutableArray<NSString *> *qrStrings = [NSMutableArray array];
    for (NSUInteger i = 0; i < chunkCount; i++) {
        NSString *qrString = [encoder getQRStringAtIndex:i error:&error];
        TEST_ASSERT(qrString != nil, ([NSString stringWithFormat:@"Failed to get QR string at %lu: %@", i, error]));
        [qrStrings addObject:qrString];
    }

    // Process in reverse order
    for (NSInteger i = chunkCount - 1; i >= 0; i--) {
        AGQRResult *qrResult = [decoder processQRString:qrStrings[i] error:&error];
        TEST_ASSERT(qrResult != nil, ([NSString stringWithFormat:@"Failed to process chunk in reverse: %@", error]));
    }

    TEST_ASSERT(decoder.isComplete, @"Decoder should be complete after processing all chunks");

    NSData *decodedData = [decoder getDataWithError:&error];
    TEST_ASSERT([decodedData isEqualToData:originalData], @"Decoded data does not match when processed out of order");

    NSLog(@"Successfully processed %lu chunks out of order", (unsigned long)chunkCount);

    return YES;
}

BOOL testDecoderReset(void) {
    NSLog(@"\n=== Testing Decoder Reset ===");

    NSString *testString = @"Reset test";
    NSData *originalData = [testString dataUsingEncoding:NSUTF8StringEncoding];

    NSError *error = nil;
    AGEncoder *encoder = [[AGEncoder alloc] initWithData:originalData chunkSize:100 error:&error];
    AGDecoder *decoder = [[AGDecoder alloc] init];

    // Process first chunk
    NSString *qrString = [encoder getQRStringAtIndex:0 error:&error];
    [decoder processQRString:qrString error:&error];

    TEST_ASSERT(decoder.receivedChunks == 1, @"Should have received 1 chunk");

    // Reset decoder
    [decoder reset];

    TEST_ASSERT(decoder.receivedChunks == 0, @"Received chunks should be 0 after reset");
    TEST_ASSERT(decoder.totalChunks == 0, @"Total chunks should be 0 after reset");
    TEST_ASSERT(!decoder.isComplete, @"Decoder should not be complete after reset");
    TEST_ASSERT(decoder.sessionId == -1, @"Session ID should be -1 after reset");

    NSLog(@"Decoder successfully reset");

    return YES;
}

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        NSLog(@"\n🧪 Running Airgap ObjC Tests\n");

        BOOL allPassed = YES;

        allPassed &= testBasicRoundtrip();
        allPassed &= testLargeDataRoundtrip();
        allPassed &= testOutOfOrderChunks();
        allPassed &= testDecoderReset();

        if (allPassed) {
            NSLog(@"\n✅ All tests passed!");
            return 0;
        } else {
            NSLog(@"\n❌ Some tests failed");
            return 1;
        }
    }
}