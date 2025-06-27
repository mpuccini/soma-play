#!/bin/bash
# SomaFM Player Installation Script
# Usage: curl -sSL https://raw.githubusercontent.com/mpuccini/soma-play/main/install.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="mpuccini/soma-play"
BINARY_NAME="soma-player"
INSTALL_DIR="$HOME/.local/bin"

# Functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

detect_platform() {
    local os
    local arch
    
    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    arch=$(uname -m)
    
    case $os in
        linux)
            case $arch in
                x86_64)
                    echo "linux-x64"
                    ;;
                *)
                    log_error "Unsupported architecture: $arch"
                    log_error "Currently supported: x86_64 (Linux), arm64 (macOS)"
                    exit 1
                    ;;
            esac
            ;;
        darwin)
            case $arch in
                arm64)
                    echo "macos-arm64"
                    ;;
                *)
                    log_error "Unsupported macOS architecture: $arch"
                    log_error "Currently supported: arm64 (Apple Silicon)"
                    exit 1
                    ;;
            esac
            ;;
        *)
            log_error "Unsupported operating system: $os"
            log_error "Currently supported: Linux (x86_64), macOS (arm64)"
            exit 1
            ;;
    esac
}

get_latest_version() {
    local api_url="https://api.github.com/repos/$REPO/releases/latest"
    
    if command -v curl >/dev/null 2>&1; then
        curl -s "$api_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$api_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    else
        log_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi
}

download_and_install() {
    local platform=$1
    local version=$2
    local archive_name="${BINARY_NAME}-${platform}.tar.gz"
    local download_url="https://github.com/$REPO/releases/download/$version/$archive_name"
    local temp_dir
    
    temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT
    
    log_info "Downloading $BINARY_NAME $version for $platform..."
    
    cd "$temp_dir"
    if command -v curl >/dev/null 2>&1; then
        curl -L -o "$archive_name" "$download_url"
    elif command -v wget >/dev/null 2>&1; then
        wget -O "$archive_name" "$download_url"
    fi
    
    if [ ! -f "$archive_name" ]; then
        log_error "Failed to download $archive_name"
        exit 1
    fi
    
    log_info "Extracting archive..."
    tar -xzf "$archive_name"
    
    if [ ! -f "$BINARY_NAME" ]; then
        log_error "Binary not found in archive"
        exit 1
    fi
    
    # Make sure install directory exists
    mkdir -p "$INSTALL_DIR"
    
    # Install binary
    log_info "Installing to $INSTALL_DIR/$BINARY_NAME..."
    mv "$BINARY_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    
    log_info "Installation completed successfully!"
}

check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        log_warn "$INSTALL_DIR is not in your PATH"
        log_warn "Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo ""
        echo "export PATH=\"\$PATH:$INSTALL_DIR\""
        echo ""
        log_warn "Or run: export PATH=\"\$PATH:$INSTALL_DIR\" (for current session only)"
    fi
}

verify_installation() {
    if [ -x "$INSTALL_DIR/$BINARY_NAME" ]; then
        log_info "Verification: $BINARY_NAME is installed at $INSTALL_DIR/$BINARY_NAME"
        
        # Try to get version (with timeout to avoid hanging)
        if timeout 5s "$INSTALL_DIR/$BINARY_NAME" --version >/dev/null 2>&1; then
            local version_output
            version_output=$(timeout 5s "$INSTALL_DIR/$BINARY_NAME" --version 2>/dev/null || echo "Version check failed")
            log_info "Version: $version_output"
        else
            log_warn "Version check failed or timed out, but binary is installed"
        fi
        
        return 0
    else
        log_error "Installation verification failed"
        return 1
    fi
}

main() {
    log_info "Starting SomaFM Player installation..."
    
    # Detect platform
    local platform
    platform=$(detect_platform)
    log_info "Detected platform: $platform"
    
    # Get latest version
    local version
    version=$(get_latest_version)
    if [ -z "$version" ]; then
        log_error "Failed to get latest version from GitHub"
        exit 1
    fi
    log_info "Latest version: $version"
    
    # Download and install
    download_and_install "$platform" "$version"
    
    # Verify installation
    if verify_installation; then
        log_info "✅ Installation successful!"
        echo ""
        log_info "You can now run: $BINARY_NAME"
        echo ""
        check_path
    else
        log_error "❌ Installation failed"
        exit 1
    fi
}

# Run main function
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
