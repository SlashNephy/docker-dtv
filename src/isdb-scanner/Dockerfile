# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260724110457@sha256:6d422783ffedba8e818e22186020897113e24b47d6ad72254a33c3c6cc98b545

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
