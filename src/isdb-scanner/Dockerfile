# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260114020209@sha256:a528842dff089f934596ad6c262b82c606e0054942c7ac346432e6484b1b0fd4

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
