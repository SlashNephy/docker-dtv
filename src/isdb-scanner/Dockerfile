# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20261005070505@sha256:f2f86b557969a3750048cdcbf97894b9a39ebb2e1945d3b86386a0647d9fd506

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
