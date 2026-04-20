# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260420185613@sha256:b0b6250e3de10c43fbdc2b22b6ed825ff8f6f405e3e9b847fbf47a4dbd9fe007

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
