# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260510020710@sha256:252ee77108a4c919d8b2e9074c9408d8ae1a24c954ecb6f7893fdb939a2a046e

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
