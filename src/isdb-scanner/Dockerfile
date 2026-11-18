# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20261116033318@sha256:07befc6f4cf8b06a436ff734172d40146d827c096c8322bb516a7cd733bad9e0

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
