# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260819000014@sha256:cca702e0a66f4afa46075745c36c6c8ea21e1adc810500c3c2d604c26b066c8f

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
