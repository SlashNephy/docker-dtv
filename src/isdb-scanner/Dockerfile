# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260216105513@sha256:8e579e7f3ec6f8baa2c0403dfc017427c9cfbe86d1401dc9b77e9757c286819c

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
