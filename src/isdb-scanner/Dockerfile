# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260313102339@sha256:418224b2456527db406a38769d8e640c8be03343f04b7d08515673ce8dbfc3a2

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
