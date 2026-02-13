# Add project specific ProGuard rules here.

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep Airgap classes
-keep class app.rkz.airgap.** { *; }