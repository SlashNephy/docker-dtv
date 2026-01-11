# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/QSVEnc versioning=regex:^(?<major>\d+)\.(?<minor>\d+)$
ARG QSVENCC_VERSION="8.04"

FROM alpine:3.23.2@sha256:865b95f46d98cf867a156fe4a135ad3fe50d2056aa3f25ed31662dff6da4eb62 AS downloader
RUN apk add --no-cache github-cli

FROM downloader AS package
ARG QSVENCC_VERSION
RUN gh release download "${QSVENCC_VERSION}" --repo rigaya/QSVEnc -p '*_Ubuntu24.04_amd64.deb' -O /package.deb

FROM ubuntu:noble-20251013@sha256:c35e29c9450151419d9448b0fd75374fec4fff364a27f176fb458d472dfc9e54

COPY --from=package /package.deb /tmp/package.deb

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends /tmp/package.deb

  apt-get clean
  rm -rf \
    /tmp/* \
    /var/lib/apt/lists
EOF

ENTRYPOINT [ "/usr/bin/qsvencc" ]
