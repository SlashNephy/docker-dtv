# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20261030070639@sha256:60b947c8beb794762896569377a60bcc703671b22f2201369f965901002a025b

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
