# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20261014054016@sha256:a32cc775a7337cc74bb503d999df3288e20ba6614f0612006382054095712a5b

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
