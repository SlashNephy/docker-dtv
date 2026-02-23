# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260223013928@sha256:beae8bb4e0082ce8dd1a623b5a69d1384ec3d12e2ee6c349710ec8e78d664149

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
