# syntax=docker/dockerfile:1

FROM ghcr.io/slashnephy/recisdb-rs:20260225194256@sha256:a518dbf03f53e914c78ff32d693fb552382e711695d7c754716e5e8de314de73

# renovate: datasource=github-tags depName=tsukumijima/ISDBScanner
ARG ISDB_SCANNER_VERSION="v1.3.3"

ADD https://github.com/tsukumijima/ISDBScanner/releases/download/${ISDB_SCANNER_VERSION}/isdb-scanner /usr/local/bin/isdb-scanner

ENTRYPOINT [ "/usr/local/bin/isdb-scanner" ]
