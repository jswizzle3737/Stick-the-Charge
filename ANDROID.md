# Android build

The Android app wraps the existing Rust and WebAssembly mobile interface with
Capacitor. It uses application ID `com.jswizzle.stickthecharge`. Decide on a
permanent application ID before publishing: an app already published under an
ID cannot be updated under a different one.

Install Node.js 22 or newer, Android Studio with the Android SDK, and a JDK
compatible with the Gradle version in `android/gradle/wrapper`. From the root:

```powershell
npm ci
npm run android:sync
cd android
.\gradlew.bat assembleDebug
```

The installable test package is at
`android/app/build/outputs/apk/debug/app-debug.apk`. Debug packages use a debug
signing key and are not production releases. For the Play Store, build and sign
a release Android App Bundle using Android Studio and protect the upload key.

To update the Rust interface, follow `mobile/README.md` to rebuild the
WebAssembly package, then run `npm run android:sync` and rebuild the APK.
The Android project requests camera access for the photo feature, and Android
backup is disabled for app data. Verify camera capture, photo export, external
links, saved charges, and offline startup on an actual Android phone before
distributing the app. Uninstalling can remove locally saved photos and charges.
