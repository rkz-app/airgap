package app.rkz.airgap

/**
 * Exception thrown when an Airgap operation fails
 */
class AirgapException : Exception {
    constructor(message: String) : super(message)
    constructor(message: String, cause: Throwable?) : super(message, cause)
}