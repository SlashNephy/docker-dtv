# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260708184417@sha256:3d9d7a1c7ba10b380c936dc9f4cfbe5d21b3b1b0db27a1471ede0d64950be93f

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
