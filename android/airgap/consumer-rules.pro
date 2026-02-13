# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep Airgap classes
-keep class app.rkz.airgap.** { *; }