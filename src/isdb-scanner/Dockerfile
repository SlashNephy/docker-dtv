# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260622180344@sha256:17d4e2d809be32582ed97867f41cf5c196ffbbea0d02f3d3e07329ca52970f79

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
