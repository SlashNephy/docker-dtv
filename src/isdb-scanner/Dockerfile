# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260104223143@sha256:bc83bb71c1027e4b56d80f4c6c130875fc3cf3ed530a768cdb77d55d405595e2

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
