# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260722040204@sha256:24fbefea76f2b78d747257d87a5b940cfa79aef92b8b33169e8af10da7c507e9

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
