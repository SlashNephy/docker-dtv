# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260111222728@sha256:a5c5c125b6fd1f27bd4e1d4907b644d8101b96a807665e7c9c1071dfe4d09929

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
