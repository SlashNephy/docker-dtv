# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260702154309@sha256:642bbe55dda26f26b4e6670dc60db5576abe1134bd40e2807b9f263d6c07640a

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
