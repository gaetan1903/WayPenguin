#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

PACKAGE_NAME="waypenguin-daemon"
VERSION="$(awk -F '"' '/^version = / { print $2; exit }' waypenguin-daemon/Cargo.toml)"
ARCHES="${RELEASE_ARCHES:-x86_64 arm64}"
STRICT_MODE="${RELEASE_STRICT:-0}"
OUT_DIR="release/${VERSION}"

if [[ -z "$VERSION" ]]; then
    echo "error: could not detect version from waypenguin-daemon/Cargo.toml" >&2
    exit 1
fi

mkdir -p "$OUT_DIR"

host_arch_normalized() {
    case "$(uname -m)" in
        x86_64|amd64) echo "x86_64" ;;
        aarch64|arm64) echo "arm64" ;;
        *) echo "unknown" ;;
    esac
}

report_issue() {
    local message="$1"
    echo "warning: ${message}" >&2
    if [[ "$STRICT_MODE" == "1" ]]; then
        had_errors=1
    else
        had_warnings=1
    fi
}

rust_target_for_arch() {
    case "$1" in
        x86_64) echo "x86_64-unknown-linux-gnu" ;;
        arm64) echo "aarch64-unknown-linux-gnu" ;;
        *) return 1 ;;
    esac
}

deb_arch_for_arch() {
    case "$1" in
        x86_64) echo "amd64" ;;
        arm64) echo "arm64" ;;
        *) return 1 ;;
    esac
}

rpm_arch_for_arch() {
    case "$1" in
        x86_64) echo "x86_64" ;;
        arm64) echo "aarch64" ;;
        *) return 1 ;;
    esac
}

ensure_rust_target() {
    local rust_target="$1"
    if command -v rustup >/dev/null 2>&1; then
        if ! rustup target list --installed | grep -qx "$rust_target"; then
            echo "==> Installing Rust target ${rust_target}"
            rustup target add "$rust_target"
        fi
    fi
}

cross_toolchain_ready() {
    local rust_target="$1"
    local arch="$2"
    local host_arch
    host_arch="$(host_arch_normalized)"

    if [[ "$host_arch" == "$arch" ]]; then
        return 0
    fi

    if [[ -n "${PKG_CONFIG:-}" || -n "${PKG_CONFIG_SYSROOT_DIR:-}" || -n "${PKG_CONFIG_PATH:-}" ]]; then
        return 0
    fi

    case "$arch" in
        arm64)
            command -v aarch64-linux-gnu-pkg-config >/dev/null 2>&1 || return 1
            command -v aarch64-linux-gnu-gcc >/dev/null 2>&1 || return 1
            ;;
        x86_64)
            command -v x86_64-linux-gnu-pkg-config >/dev/null 2>&1 || return 1
            command -v x86_64-linux-gnu-gcc >/dev/null 2>&1 || return 1
            ;;
        *)
            return 1
            ;;
    esac

    return 0
}

had_errors=0
had_warnings=0
for arch in $ARCHES; do
    rust_target="$(rust_target_for_arch "$arch" || true)"
    if [[ -z "$rust_target" ]]; then
        report_issue "unsupported arch '${arch}', skipping"
        continue
    fi

    if ! cross_toolchain_ready "$rust_target" "$arch"; then
        report_issue "cross toolchain/sysroot for ${arch} is not configured; skipping ${arch}."
        echo "         hint: configure PKG_CONFIG_SYSROOT_DIR + PKG_CONFIG_PATH, or install ${arch} cross gcc/pkg-config wrappers" >&2
        continue
    fi

    echo "==> Building release binary for ${arch} (${rust_target})"
    ensure_rust_target "$rust_target"
    if ! cargo build --release -p "$PACKAGE_NAME" --target "$rust_target"; then
        report_issue "failed to build ${arch}; check cross-compiler/linker setup"
        continue
    fi

    BIN_SRC="target/${rust_target}/release/${PACKAGE_NAME}"
    BIN_DST="${OUT_DIR}/${PACKAGE_NAME}-linux-${arch}"
    cp "$BIN_SRC" "$BIN_DST"
    chmod +x "$BIN_DST"

    echo "==> Creating tarball for ${arch}"
    TARBALL="${OUT_DIR}/${PACKAGE_NAME}-${VERSION}-linux-${arch}.tar.gz"
    tar -C "$OUT_DIR" -czf "$TARBALL" "${PACKAGE_NAME}-linux-${arch}"

    if command -v cargo-deb >/dev/null 2>&1; then
        echo "==> Building .deb package for ${arch}"
        if cargo deb -p "$PACKAGE_NAME" --no-build --target "$rust_target"; then
            deb_arch="$(deb_arch_for_arch "$arch")"
            DEB_FILE="$(find target -type f -name "*.deb" | grep "_${deb_arch}\\.deb$" | head -n 1 || true)"
            if [[ -n "$DEB_FILE" ]]; then
                cp "$DEB_FILE" "$OUT_DIR/"
            else
                report_issue ".deb build finished but no ${deb_arch} .deb file was found"
            fi
        else
            report_issue ".deb build failed for ${arch}"
        fi
    else
        report_issue "cargo-deb is not installed, skipping .deb build for ${arch}"
        echo "         install with: cargo install cargo-deb" >&2
    fi

    if command -v cargo-generate-rpm >/dev/null 2>&1; then
        echo "==> Building .rpm package for ${arch}"
        if cargo generate-rpm -p "$PACKAGE_NAME" --target "$rust_target"; then
            rpm_arch="$(rpm_arch_for_arch "$arch")"
            RPM_FILE="$(find target -type f -name "*.rpm" | grep "\\.${rpm_arch}\\.rpm$" | head -n 1 || true)"
            if [[ -n "$RPM_FILE" ]]; then
                cp "$RPM_FILE" "$OUT_DIR/"
            else
                report_issue ".rpm build finished but no ${rpm_arch} .rpm file was found"
            fi
        else
            report_issue ".rpm build failed for ${arch}"
        fi
    else
        report_issue "cargo-generate-rpm is not installed, skipping .rpm build for ${arch}"
        echo "         install with: cargo install cargo-generate-rpm" >&2
    fi
done

echo "==> Done"
echo "Artifacts available in: ${OUT_DIR}"
ls -1 "$OUT_DIR"

if [[ "$had_errors" -ne 0 ]]; then
    echo "error: one or more architectures/packages failed (strict mode). See logs above." >&2
    exit 1
fi

if [[ "$had_warnings" -ne 0 ]]; then
    echo "warning: some artifacts were skipped or failed, but native artifacts were produced." >&2
    echo "         set RELEASE_STRICT=1 to make these conditions fail the script." >&2
fi
