# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260423103225@sha256:c951c04045b4a243f86949830590233b910cd63bb7bb88158c72180ccad652ce

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
