# Maintainer: deadnews <deadnewsgit@gmail.com>

_pkgname="pindock"
pkgname="${_pkgname}-bin"
pkgver="0.0.0"
pkgrel=1
pkgdesc="Pin and update Docker image digests in Dockerfiles and compose files"
arch=("x86_64" "aarch64")
url="https://github.com/deadnews/pindock"
license=("MIT")
provides=("pindock")
conflicts=("pindock")
options=("!strip")

source_x86_64=("${_pkgname}_${pkgver//_/-}_linux_amd64.tar.gz::${url}/releases/download/v${pkgver//_/-}/${_pkgname}_${pkgver//_/-}_linux_amd64.tar.gz")
source_aarch64=("${_pkgname}_${pkgver//_/-}_linux_arm64.tar.gz::${url}/releases/download/v${pkgver//_/-}/${_pkgname}_${pkgver//_/-}_linux_arm64.tar.gz")

sha256sums_x86_64=("SKIP")
sha256sums_aarch64=("SKIP")

package() {
    install -Dm755 "${srcdir}/${_pkgname}" "${pkgdir}/usr/bin/${_pkgname}"
}
