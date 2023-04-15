# Maintainer: mhdy <https://gitlab.com/mhdy>
pkgname=mblocks
pkgver=0.1.0
pkgrel=1
pkgdesc="multithreated, async statusbar for dwm and likes, written in Rust"
makedepends=('rust' 'cargo' 'git')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
url="https://github.com/Ordained-SubGenii/mblocks-custom"
license=('MIT')
source=("git+$url")
sha256sums=('SKIP')
# Generated in accordance to https://wiki.archlinux.org/title/Rust_package_guidelines.
# Might require further modification depending on the package involved.
prepare() {
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --all-features
}

package() {
  install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}
