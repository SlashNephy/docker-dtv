# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260902073515@sha256:d551b7a0dc3d7860f88ba3d9176420b8fdf5f2e44b4efdebfda54053d37dbd99

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
