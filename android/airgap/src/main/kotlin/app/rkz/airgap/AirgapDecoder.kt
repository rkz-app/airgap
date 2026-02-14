package app.rkz.airgap

/**
 * AirgapDecoder decodes QR code chunks back into the original data
 */
class AirgapDecoder : AutoCloseable {

    private var nativeHandle: Long = 0

    init {
        System.loadLibrary("airgap")
        nativeHandle = nativeNew()
        if (nativeHandle == 0L) {
            throw AirgapException("Failed to create decoder")
        }
    }

    /**
     * Whether all chunks have been received and the data is complete
     */
    val isComplete: Boolean
        get() {
            checkNotClosed()
            return nativeIsComplete(nativeHandle)
        }

    /**
     * Total number of chunks expected (0 if not yet known)
     */
    val totalChunks: Int
        get() {
            checkNotClosed()
            return nativeGetTotal(nativeHandle)
        }

    /**
     * Number of unique chunks received so far
     */
    val receivedChunks: Int
        get() {
            checkNotClosed()
            return nativeGetReceived(nativeHandle)
        }

    /**
     * The session ID of the current decoding session (-1 if no session started)
     */
    val sessionId: Int
        get() {
            checkNotClosed()
            return nativeGetSessionId(nativeHandle)
        }

    /**
     * Get the decoding progress as a pair of (received, total)
     */
    val progress: Pair<Int, Int>
        get() = Pair(receivedChunks, totalChunks)

    /**
     * Process a QR code string
     *
     * @param qrString The string data from a scanned QR code
     * @return QRResult with chunk information
     * @throws AirgapException if processing fails
     */
    @Throws(AirgapException::class)
    fun processQrString(qrString: String): QRResult {
        checkNotClosed()
        // JNI will throw AirgapException on error
        return nativeProcessQr(nativeHandle, qrString)
            ?: throw AirgapException("Failed to process QR code")
    }

    /**
     * Reset the decoder to its initial state
     */
    fun reset() {
        checkNotClosed()
        nativeReset(nativeHandle)
    }

    /**
     * Get the decoded data once complete
     *
     * @return The decoded data
     * @throws AirgapException if decoding is not complete or retrieval fails
     */
    @Throws(AirgapException::class)
    fun getData(): ByteArray {
        checkNotClosed()
        // JNI will throw AirgapException on error (including if not complete)
        return nativeGetData(nativeHandle)
            ?: throw AirgapException("Failed to retrieve decoded data")
    }

    override fun close() {
        if (nativeHandle != 0L) {
            nativeFree(nativeHandle)
            nativeHandle = 0
        }
    }

    private fun checkNotClosed() {
        if (nativeHandle == 0L) {
            throw IllegalStateException("Decoder has been closed")
        }
    }

    // Native methods
    private external fun nativeNew(): Long
    private external fun nativeFree(handle: Long)
    private external fun nativeIsComplete(handle: Long): Boolean
    private external fun nativeGetTotal(handle: Long): Int
    private external fun nativeGetReceived(handle: Long): Int
    private external fun nativeGetSessionId(handle: Long): Int
    private external fun nativeProcessQr(handle: Long, qrString: String): QRResult?
    private external fun nativeGetData(handle: Long): ByteArray?
    private external fun nativeReset(handle: Long)
}