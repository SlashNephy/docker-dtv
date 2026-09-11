# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260909164345@sha256:f4ae8c4bbad887b4ce023a0c53b7c6ac64bb685c9844b413f4bf32809e772900

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
