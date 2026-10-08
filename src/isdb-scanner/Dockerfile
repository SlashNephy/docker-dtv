# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20261008111513@sha256:c3c45959fce9438aeb40a43e8a374293584e253f9e954666fe480993457a6d34

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
