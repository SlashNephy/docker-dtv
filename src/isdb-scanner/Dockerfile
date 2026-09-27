# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260927033558@sha256:3ee255d7b64aa87df9dbcf9a9612f312893bd8ae7f3697c320b7844eec9511ce

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
