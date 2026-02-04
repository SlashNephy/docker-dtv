# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260204065109@sha256:ed1ab2e876f72a1339e9cb268933e513877e4c2d8ec69a29780b0e01c0613e1a

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
