package app.rkz.airgap

/**
 * Result from processing a QR code chunk
 *
 * @property chunkNumber The chunk number that was processed (0-based index)
 * @property totalChunks The total number of chunks in this session
 */
data class QRResult(
    val chunkNumber: Int,
    val totalChunks: Int
)
