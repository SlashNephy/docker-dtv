# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260202025049@sha256:b77759cadc6dc99f5e26b189d5c204fd1711afff5fec0b9528e9dbe21c6bc4fb

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
