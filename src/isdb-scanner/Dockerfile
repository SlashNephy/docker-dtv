# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260211132521@sha256:66323559e31f567aea3736bf9c9a228f9f049d5ec12de7cadc62b3e1a42acba4

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
