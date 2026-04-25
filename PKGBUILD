# Maintainer: CINAR <cinar@email.com>
# Package: karga-bin

pkgname=karga-bin
_pkgname=karga
pkgver=0.1.0
pkgrel=1
pkgdesc="Türkçe Terminal Haber Okuyucu - RSS News Reader"
arch=('x86_64')
url="https://github.com/cinar/karga"
license=('MIT')
provides=('karga')
conflicts=('karga-git')
depends=('gcc-libs' 'glibc' 'libc' 'openssl' 'zlib')
source=("karga-${pkgver}-x86_64.tar.gz::https://github.com/cinar/karga/releases/download/v${pkgver}/karga-${pkgver}-x86_64.tar.gz")
sha256sums=('SKIP')

package() {
    install -Dm755 karga -t "${pkgdir}/usr/bin/"
}