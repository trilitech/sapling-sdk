# Sapling Android



An Android wrapper around [Zcash Rust crates](https://github.com/zcash/librustzcash).

## Install

To add the Sapling Android library into your project:

1. Ensure [Android NDK](https://developer.android.com/ndk) is supported in your project. 

2. Add the [JitPack](https://jitpack.io/) repository to your root `build.gradle` file:
  ```groovy
  allprojects {
    repositories {
      ...
      maven { url 'https://jitpack.io' }
    }
  }
  ```

## Development

### Update C Bindings
To update the C bindings from the core `sapling` package run:
```bash
# $ pwd
# <project-dir>/packages/sapling-android

$ ./build-ffi.sh
```
