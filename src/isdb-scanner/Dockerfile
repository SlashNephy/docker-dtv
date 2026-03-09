# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260309135217@sha256:030f88760e410f000758f5f35763a69f6f552edd773fcc18f8ca9fa46aab6857

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
