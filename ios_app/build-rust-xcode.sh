#!/bin/bash

# Lifted from xi-mac editor

# When building from Xcode we want to ensure that `cargo` is in PATH.
# as a convenience, add the default cargo install location
export PATH="$PATH:${HOME}/.cargo/bin"

set -e
set -x


function build_target () {
    TARGET_NAME="$1"
    CARGO_PROJECT_FLAG="$2"
    cd "${SRCROOT}/../"
    if [[ ${ACTION:-build} = "build" ]]; then
        if [[ $PLATFORM_NAME = "" ]]; then
            # default for building with xcodebuild
            PLATFORM_NAME="macosx"
        fi

        if [[ $PLATFORM_NAME = "macosx" ]]; then
            RUST_TARGET_OS="darwin"
        else
            RUST_TARGET_OS="ios"
        fi

        for ARCH in $ARCHS
        do
            if [[ $(lipo -info "${BUILT_PRODUCTS_DIR}/${TARGET_NAME}" 2>&1) != *"${ARCH}"* ]]; then
                rm -f "${BUILT_PRODUCTS_DIR}/${TARGET_NAME}"
            fi
        done

        if [[ $CONFIGURATION = "Debug" ]]; then
            RUST_CONFIGURATION="debug"
            RUST_CONFIGURATION_FLAG=""
        else
            RUST_CONFIGURATION="release"
            RUST_CONFIGURATION_FLAG="--release"
        fi

        EXECUTABLES=()
        for ARCH in $ARCHS
        do
            RUST_ARCH=$ARCH
            if [[ $RUST_ARCH = "arm64" ]]; then
                RUST_ARCH="aarch64"
            fi
            env
            cargo build $RUST_CONFIGURATION_FLAG $CARGO_PROJECT_FLAG --target "${RUST_ARCH}-apple-${RUST_TARGET_OS}"
            EXECUTABLES+=("target/${RUST_ARCH}-apple-${RUST_TARGET_OS}/${RUST_CONFIGURATION}/${TARGET_NAME}")
        done

        #mkdir -p "${BUILT_PRODUCTS_DIR}"
        #xcrun --sdk $PLATFORM_NAME lipo -create "${EXECUTABLES[@]}" -output "${BUILT_PRODUCTS_DIR}/${TARGET_NAME}"
    elif [[ $ACTION = "clean" ]]; then
        cargo clean
        rm -f "${BUILT_PRODUCTS_DIR}/${TARGET_NAME}"
    fi
}

build_target mobcore ""

set +e
set +x
