package app.rkz.airgap

import kotlin.test.*

/**
 * Roundtrip tests for Airgap encoder and decoder
 */
class AirgapTests {

    @Test
    fun testBasicRoundtrip() {
        println("\n=== Testing Basic Roundtrip ===")

        // Create test data
        val testString = "Hello, Airgap!"
        val originalData = testString.toByteArray(Charsets.UTF_8)

        // Create encoder
        val encoder = AirgapEncoder(originalData, chunkSize = 100)
        assertNotNull(encoder, "Encoder should not be null")

        // Check chunk count
        val chunkCount = encoder.chunkCount
        println("Chunk count: $chunkCount")
        assertTrue(chunkCount > 0, "Should have at least one chunk")

        // Get session ID
        val sessionId = encoder.sessionId
        println("Session ID: $sessionId")
        assertTrue(sessionId != 0, "Session ID should not be zero")

        // Create decoder
        val decoder = AirgapDecoder()
        assertNotNull(decoder, "Decoder should not be null")

        // Process all chunks
        for (i in 0 until chunkCount) {
            // Get QR string (this is what would be scanned from a QR code)
            val qrString = encoder.getQRString(i)
            assertNotNull(qrString, "QR string at index $i should not be null")
            assertTrue(qrString.isNotEmpty(), "QR string should not be empty")

            // Process in decoder
            val qrResult = decoder.processQrString(qrString)
            assertNotNull(qrResult, "QRResult should not be null")
            assertEquals(i, qrResult.chunkNumber, "Chunk number mismatch")
            assertEquals(chunkCount, qrResult.totalChunks, "Total chunks mismatch")

            println("Processed chunk ${qrResult.chunkNumber + 1}/${qrResult.totalChunks}")
        }

        // Check decoder is complete
        assertTrue(decoder.isComplete, "Decoder should be complete")
        assertEquals(chunkCount, decoder.receivedChunks, "Received chunks mismatch")
        assertEquals(chunkCount, decoder.totalChunks, "Total chunks mismatch")
        assertEquals(sessionId, decoder.sessionId, "Session ID mismatch")

        // Get decoded data
        val decodedData = decoder.getData()
        assertNotNull(decodedData, "Decoded data should not be null")
        assertEquals(originalData.size, decodedData.size, "Decoded data length mismatch")
        assertContentEquals(originalData, decodedData, "Decoded data does not match original")

        val decodedString = decodedData.toString(Charsets.UTF_8)
        println("Original: $testString")
        println("Decoded:  $decodedString")
        assertEquals(testString, decodedString, "Decoded string does not match original")

        // Cleanup
        encoder.close()
        decoder.close()
    }

    @Test
    fun testLargeDataRoundtrip() {
        println("\n=== Testing Large Data Roundtrip ===")

        // Create larger test data (5KB)
        val largeData = ByteArray(5000) { (it % 256).toByte() }

        // Create encoder with smaller chunk size to test multiple chunks
        val encoder = AirgapEncoder(largeData, chunkSize = 500)
        assertNotNull(encoder, "Encoder should not be null")

        val chunkCount = encoder.chunkCount
        println("Large data chunk count: $chunkCount")
        assertTrue(chunkCount > 5, "Should have multiple chunks")

        // Create decoder
        val decoder = AirgapDecoder()
        assertNotNull(decoder, "Decoder should not be null")

        // Process all chunks in order
        for (i in 0 until chunkCount) {
            val qrString = encoder.getQRString(i)
            assertNotNull(qrString, "QR string at index $i should not be null")

            val qrResult = decoder.processQrString(qrString)
            assertNotNull(qrResult, "QRResult at index $i should not be null")
        }

        assertTrue(decoder.isComplete, "Decoder should be complete")

        // Get and verify decoded data
        val decodedData = decoder.getData()
        assertNotNull(decodedData, "Decoded data should not be null")
        assertContentEquals(largeData, decodedData, "Decoded data does not match original")

        println("Successfully roundtripped ${largeData.size} bytes in $chunkCount chunks")

        // Cleanup
        encoder.close()
        decoder.close()
    }

    @Test
    fun testOutOfOrderChunks() {
        println("\n=== Testing Out of Order Chunks ===")

        // Create test data
        val testString = "Out of order test data"
        val originalData = testString.toByteArray(Charsets.UTF_8)

        val encoder = AirgapEncoder(originalData, chunkSize = 50)
        assertNotNull(encoder, "Encoder should not be null")

        val chunkCount = encoder.chunkCount
        val decoder = AirgapDecoder()

        // Collect all QR strings first
        val qrStrings = mutableListOf<String>()
        for (i in 0 until chunkCount) {
            val qrString = encoder.getQRString(i)
            assertNotNull(qrString, "QR string at index $i should not be null")
            qrStrings.add(qrString)
        }

        // Process in reverse order
        for (i in chunkCount - 1 downTo 0) {
            val qrResult = decoder.processQrString(qrStrings[i])
            assertNotNull(qrResult, "QRResult should not be null when processing in reverse")
        }

        assertTrue(decoder.isComplete, "Decoder should be complete after processing all chunks")

        val decodedData = decoder.getData()
        assertContentEquals(originalData, decodedData, "Decoded data does not match when processed out of order")

        println("Successfully processed $chunkCount chunks out of order")

        // Cleanup
        encoder.close()
        decoder.close()
    }

    @Test
    fun testDecoderReset() {
        println("\n=== Testing Decoder Reset ===")

        val testString = "Reset test"
        val originalData = testString.toByteArray(Charsets.UTF_8)

        val encoder = AirgapEncoder(originalData, chunkSize = 100)
        val decoder = AirgapDecoder()

        // Process first chunk
        val qrString = encoder.getQRString(0)
        decoder.processQrString(qrString)

        assertEquals(1, decoder.receivedChunks, "Should have received 1 chunk")

        // Reset decoder
        decoder.reset()

        assertEquals(0, decoder.receivedChunks, "Received chunks should be 0 after reset")
        assertEquals(0, decoder.totalChunks, "Total chunks should be 0 after reset")
        assertFalse(decoder.isComplete, "Decoder should not be complete after reset")
        assertEquals(-1, decoder.sessionId, "Session ID should be -1 after reset")

        println("Decoder successfully reset")

        // Cleanup
        encoder.close()
        decoder.close()
    }

    @Test
    fun testMultipleEncoders() {
        println("\n=== Testing Multiple Encoders ===")

        val data1 = "First encoder data".toByteArray(Charsets.UTF_8)
        val data2 = "Second encoder data".toByteArray(Charsets.UTF_8)

        val encoder1 = AirgapEncoder(data1, chunkSize = 100)
        val encoder2 = AirgapEncoder(data2, chunkSize = 100)

        // Session IDs should be different
        assertNotEquals(encoder1.sessionId, encoder2.sessionId, "Session IDs should be different")

        val decoder1 = AirgapDecoder()
        val decoder2 = AirgapDecoder()

        // Decode data from encoder1
        for (i in 0 until encoder1.chunkCount) {
            decoder1.processQrString(encoder1.getQRString(i))
        }

        // Decode data from encoder2
        for (i in 0 until encoder2.chunkCount) {
            decoder2.processQrString(encoder2.getQRString(i))
        }

        assertTrue(decoder1.isComplete, "Decoder 1 should be complete")
        assertTrue(decoder2.isComplete, "Decoder 2 should be complete")

        val decoded1 = decoder1.getData()
        val decoded2 = decoder2.getData()

        assertContentEquals(data1, decoded1, "Decoded data 1 should match")
        assertContentEquals(data2, decoded2, "Decoded data 2 should match")

        println("Successfully handled multiple independent encoder/decoder pairs")

        // Cleanup
        encoder1.close()
        encoder2.close()
        decoder1.close()
        decoder2.close()
    }

    @Test
    fun testEmptyDataThrowsException() {
        println("\n=== Testing Empty Data Throws Exception ===")

        assertFailsWith<AirgapException>("Should throw exception for empty data") {
            AirgapEncoder(ByteArray(0), chunkSize = 100)
        }

        println("Empty data correctly throws exception")
    }

    @Test
    fun testInvalidChunkSizeThrowsException() {
        println("\n=== Testing Invalid Chunk Size Throws Exception ===")

        val testData = "Test".toByteArray(Charsets.UTF_8)

        assertFailsWith<AirgapException>("Should throw exception for chunk size too small") {
            AirgapEncoder(testData, chunkSize = 10)
        }

        assertFailsWith<AirgapException>("Should throw exception for chunk size too large") {
            AirgapEncoder(testData, chunkSize = 2000)
        }

        println("Invalid chunk sizes correctly throw exceptions")
    }
}

fun main() {
    println("\n🧪 Running Airgap Kotlin Tests\n")

    val tests = AirgapTests()
    var allPassed = true

    val testMethods = listOf(
        "testBasicRoundtrip" to { tests.testBasicRoundtrip() },
        "testLargeDataRoundtrip" to { tests.testLargeDataRoundtrip() },
        "testOutOfOrderChunks" to { tests.testOutOfOrderChunks() },
        "testDecoderReset" to { tests.testDecoderReset() },
        "testMultipleEncoders" to { tests.testMultipleEncoders() },
        "testEmptyDataThrowsException" to { tests.testEmptyDataThrowsException() },
        "testInvalidChunkSizeThrowsException" to { tests.testInvalidChunkSizeThrowsException() }
    )

    for ((name, test) in testMethods) {
        try {
            test()
            println("✅ PASSED: $name\n")
        } catch (e: Exception) {
            println("❌ FAILED: $name")
            println("   Error: ${e.message}\n")
            e.printStackTrace()
            allPassed = false
        }
    }

    if (allPassed) {
        println("\n✅ All tests passed!")
        System.exit(0)
    } else {
        println("\n❌ Some tests failed")
        System.exit(1)
    }
}