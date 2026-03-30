# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260330095514@sha256:b06d729cc37919bd4d4fa8479d77ad45970aa6e9893dd758482ac28ce88b9801

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
