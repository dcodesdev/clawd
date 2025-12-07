#!/bin/sh
set -e

REPO="dcodesdev/clawd"
INSTALL_DIR="${CLAWD_INSTALL_DIR:-$HOME/.local/bin}"
BINARY_NAME="clawd"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    printf "${GREEN}[INFO]${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}[WARN]${NC} %s\n" "$1"
}

error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1"
    exit 1
}

detect_os() {
    case "$(uname -s)" in
        Darwin*) echo "darwin" ;;
        Linux*)  echo "linux" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *) error "Unsupported operating system: $(uname -s)" ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64) echo "amd64" ;;
        arm64|aarch64) echo "arm64" ;;
        *) error "Unsupported architecture: $(uname -m)" ;;
    esac
}

get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" |
        grep '"tag_name":' |
        sed -E 's/.*"([^"]+)".*/\1/'
}

download_binary() {
    local os="$1"
    local arch="$2"
    local version="$3"
    local ext=""

    if [ "$os" = "windows" ]; then
        ext=".exe"
    fi

    local filename="${BINARY_NAME}-${os}-${arch}${ext}"
    local url="https://github.com/${REPO}/releases/download/${version}/${filename}"

    printf "${GREEN}[INFO]${NC} Downloading ${filename} (${version})...\n" >&2

    local tmp_dir
    tmp_dir=$(mktemp -d)
    local tmp_file="${tmp_dir}/${BINARY_NAME}${ext}"

    if ! curl -fsSL "$url" -o "$tmp_file"; then
        rm -rf "$tmp_dir"
        error "Failed to download from ${url}"
    fi

    # Verify checksum
    local checksum_url="https://github.com/${REPO}/releases/download/${version}/checksums.sha256"
    local checksums_file="${tmp_dir}/checksums.sha256"

    if curl -fsSL "$checksum_url" -o "$checksums_file" 2>/dev/null; then
        printf "${GREEN}[INFO]${NC} Verifying checksum...\n" >&2
        local expected_checksum
        expected_checksum=$(grep "$filename" "$checksums_file" | awk '{print $1}')

        if [ -n "$expected_checksum" ]; then
            local actual_checksum
            if command -v sha256sum >/dev/null 2>&1; then
                actual_checksum=$(sha256sum "$tmp_file" | awk '{print $1}')
            elif command -v shasum >/dev/null 2>&1; then
                actual_checksum=$(shasum -a 256 "$tmp_file" | awk '{print $1}')
            else
                warn "No sha256 tool found, skipping checksum verification"
                actual_checksum="$expected_checksum"
            fi

            if [ "$expected_checksum" != "$actual_checksum" ]; then
                rm -rf "$tmp_dir"
                error "Checksum verification failed!"
            fi
            printf "${GREEN}[INFO]${NC} Checksum verified!\n" >&2
        fi
    else
        warn "Could not download checksums, skipping verification"
    fi

    printf '%s' "$tmp_file"
}

install_binary() {
    local tmp_file="$1"
    local install_path="${INSTALL_DIR}/${BINARY_NAME}"

    # Create install directory if it doesn't exist
    if [ ! -d "$INSTALL_DIR" ]; then
        mkdir -p "$INSTALL_DIR"
    fi

    chmod +x "$tmp_file"

    if [ -w "$INSTALL_DIR" ]; then
        mv "$tmp_file" "$install_path"
    else
        info "Installing to ${INSTALL_DIR} requires elevated permissions..."
        sudo mv "$tmp_file" "$install_path"
    fi

    info "Installed to ${install_path}"

    # Check if install directory is in PATH
    case ":$PATH:" in
        *":$INSTALL_DIR:"*) ;;
        *)
            warn "${INSTALL_DIR} is not in your PATH"
            info "Add it by running: export PATH=\"\$PATH:${INSTALL_DIR}\""
            ;;
    esac
}

main() {
    info "Clawd Installer"
    echo ""

    local os
    local arch
    local version

    os=$(detect_os)
    arch=$(detect_arch)

    info "Detected OS: ${os}"
    info "Detected architecture: ${arch}"

    # Check for version argument or get latest
    if [ -n "$1" ]; then
        version="$1"
    else
        info "Fetching latest version..."
        version=$(get_latest_version)
    fi

    if [ -z "$version" ]; then
        error "Could not determine version to install"
    fi

    info "Version: ${version}"
    echo ""

    local tmp_file
    tmp_file=$(download_binary "$os" "$arch" "$version")

    install_binary "$tmp_file"

    # Cleanup temp directory
    rm -rf "$(dirname "$tmp_file")"

    echo ""
    info "Installation complete!"
    info "Run 'clawd --help' to get started"
}

main "$@"
