# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260127025440@sha256:9abbe5d561bcb6c0213c0812b5c5ea7b5c401bafa015fce05757610913c05fe4

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
