# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260317223219@sha256:da26441d4aecb790f884effa93591909a663fbbf2d324c00e57f6f92ad5f7f63

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
