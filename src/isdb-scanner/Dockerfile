# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260114182738@sha256:a9530c75c4d684a25634c8cf915c759d8700b1f17bb42f3c1c809623ebea2c89

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
