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
    chunkSize: Int = RECOMMENDED_MAX_CHUNK_SIZE
) : AutoCloseable {

    private var nativeHandle: Long = 0

    init {
        System.loadLibrary("airgap")

        if (data.isEmpty()) {
            throw AirgapException("Data cannot be empty")
        }

        if (chunkSize < MIN_CHUNK_SIZE || chunkSize > MAX_CHUNK_SIZE) {
            throw AirgapException(
                "Chunk size must be between $MIN_CHUNK_SIZE and $MAX_CHUNK_SIZE, got $chunkSize"
            )
        }

        nativeHandle = nativeNew(data, chunkSize)
        if (nativeHandle == 0L) {
            throw AirgapException("Failed to create encoder")
        }
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
     * Generates a PNG image for the chunk at the given index
     *
     * @param index The chunk index (0-based)
     * @return PNG image data as ByteArray
     * @throws AirgapException if generation fails or index is out of bounds
     */
    @Throws(AirgapException::class)
    fun generatePng(index: Int): ByteArray {
        checkNotClosed()

        if (index < 0 || index >= chunkCount) {
            throw AirgapException("Index $index out of bounds [0, $chunkCount)")
        }

        val result = nativeGeneratePng(nativeHandle, index)
            ?: throw AirgapException("Failed to generate PNG at index $index")

        return result
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
    private external fun nativeNew(data: ByteArray, chunkSize: Int): Long
    private external fun nativeFree(handle: Long)
    private external fun nativeChunkCount(handle: Long): Int
    private external fun nativeSessionId(handle: Long): Int
    private external fun nativeGeneratePng(handle: Long, index: Int): ByteArray?

    companion object {
        const val MIN_CHUNK_SIZE = 16
        const val MAX_CHUNK_SIZE = 1920
        const val RECOMMENDED_MAX_CHUNK_SIZE = 1100
    }
}