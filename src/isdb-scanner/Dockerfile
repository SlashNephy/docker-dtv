# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260118223247@sha256:5c39fb3be872b599b944838539ee13f4d1c2dcf579118b3932c78370a3d034ef

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
