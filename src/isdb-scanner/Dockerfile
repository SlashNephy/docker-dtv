# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260208175059@sha256:4add5304f1d25bf90230d3cae4f6461f5515877368a09e022acb916515968300

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
