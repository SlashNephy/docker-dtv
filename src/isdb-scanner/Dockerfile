# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260125214516@sha256:2c4ee8c1e8408fbc83d88fe847e30d8cc87c7b9717afbedc4365466a8530e4af

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
