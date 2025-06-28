# Maintainer: Your Name <your_email@example.com>
pkgname=huh
pkgver=0.1.0
pkgrel=1
pkgdesc="A CLI tool that provides AI-powered analysis and suggestions for your shell commands."
arch=('x86_64')
url="https://github.com/surajssc1232/wut_rust"
license=('MIT')
depends=('rust' 'cargo') # rust and cargo are build dependencies, but also needed to run the built binary
makedepends=('git') # For cloning the repository
source=("git+${url}.git") # Fetch latest commit from the repository
sha256sums=('SKIP') # For git sources, we can skip checksums

build() {
  cd "${srcdir}/wut_rust"
  cargo build --release
}

package() {
  cd "${srcdir}/wut_rust"
  install -Dm755 "target/release/wut" "${pkgdir}/usr/bin/${pkgname}"
}
