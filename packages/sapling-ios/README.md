# Sapling iOS

[![release](https://img.shields.io/github/v/tag/trilitech/sapling-sdk?include_prereleases)](https://github.com/trilitech/sapling-sdk/releases)

An iOS wrapper around [Zcash Rust crates](https://github.com/zcash/librustzcash).

## Install

To add iOS Sapling into your project, add the package dependency:

### Xcode

Open the `Add Package Dependency` window (as described in [the official guide](https://developer.apple.com/documentation/xcode/adding_package_dependencies_to_your_app)) and enter the Sapling GitHub repository URL:
```
https://github.com/trilitech/sapling-sdk
```

### Package.swift file

Add the following dependency in your `Package.swift` file:

```swift
.package(url: "https://github.com/trilitech/sapling-sdk", from: "x.y.z")
```


## Development

### Update C Bindings
To update the C bindings from the core `sapling` package run:
```bash
# $ pwd
# <project-dir>/packages/sapling-ios

$ ./build-ffi.sh
```
