# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260408020503@sha256:79e0c950924b842b58d07d8a3aa8e0d195a8632b188e32ece9187533fe91771a

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
