# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260916140059@sha256:9d8e60a0f5815f293e9687fc8bbbf1c7b7caf100385423e6b5ddc90573e6fc9a

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
