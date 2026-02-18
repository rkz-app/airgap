package app.rkz.airgap

/**
 * AirgapEncoder encodes data into QR code chunks for air-gapped communication
 *
 * @property data The data to encode
 * @property chunkSize Size of each chunk (must be between 16 and 1920 bytes, recommended: 1100)
 * @throws AirgapException if initialization fails
 */
class AirgapEncoder @Throws(AirgapException::class) constructor(
    data: ByteArray,
    chunkSize: Int = RECOMMENDED_MAX_CHUNK_SIZE,
    qrSize: Int = 400
) : AutoCloseable {

    private var nativeHandle: Long = 0

    init {
        System.loadLibrary("airgap")
        nativeHandle = nativeNew(data, chunkSize, qrSize)
    }

    /**
     * The total number of chunks this encoder will generate
     */
    val chunkCount: Int
        get() {
            checkNotClosed()
            return nativeChunkCount(nativeHandle)
        }

    /**
     * The session ID for this encoding session
     */
    val sessionId: Int
        get() {
            checkNotClosed()
            return nativeSessionId(nativeHandle)
        }

    /**
     * Gets the QR code string for the chunk at the given index (base45-encoded)
     * This is what would be scanned from a QR code
     *
     * @param index The chunk index (0-based)
     * @return QR code string (base45-encoded)
     * @throws AirgapException if retrieval fails or index is out of bounds
     */
    @Throws(AirgapException::class)
    fun getQRString(index: Int): String {
        checkNotClosed()
        // All validation is done in Rust - JNI will throw AirgapException on error
        return nativeGetQRString(nativeHandle, index)
            ?: throw AirgapException("Failed to get QR string at index $index")
    }

    /**
     * Generates a PNG image for the chunk at the given index
     *
     * @param index The chunk index (0-based)
     * @return PNG image data as ByteArray
     * @throws AirgapException if generation fails or index is out of bounds
     */
    @Throws(AirgapException::class)
    fun generatePng(index: Int): ByteArray {
        checkNotClosed()
        // All validation is done in Rust - JNI will throw AirgapException on error
        return nativeGeneratePng(nativeHandle, index)
            ?: throw AirgapException("Failed to generate PNG at index $index")
    }

    /**
     * Generates all PNG images for all chunks
     *
     * @return List of PNG image data
     * @throws AirgapException if generation fails
     */
    @Throws(AirgapException::class)
    fun generateAllPngs(): List<ByteArray> {
        checkNotClosed()
        return (0 until chunkCount).map { generatePng(it) }
    }

    override fun close() {
        if (nativeHandle != 0L) {
            nativeFree(nativeHandle)
            nativeHandle = 0
        }
    }

    private fun checkNotClosed() {
        if (nativeHandle == 0L) {
            throw IllegalStateException("Encoder has been closed")
        }
    }

    // Native methods
    private external fun nativeNew(data: ByteArray, chunkSize: Int, qrSize: Int): Long
    private external fun nativeFree(handle: Long)
    private external fun nativeChunkCount(handle: Long): Int
    private external fun nativeSessionId(handle: Long): Int
    private external fun nativeGetQRString(handle: Long, index: Int): String?
    private external fun nativeGeneratePng(handle: Long, index: Int): ByteArray?

    companion object {
        const val MIN_CHUNK_SIZE = 16
        const val MAX_CHUNK_SIZE = 1920
        const val RECOMMENDED_MAX_CHUNK_SIZE = 1100
    }
}