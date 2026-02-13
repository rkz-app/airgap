package app.rkz.airgap

/**
 * Exception thrown when an Airgap operation fails
 */
class AirgapException(message: String, cause: Throwable? = null) : Exception(message, cause)