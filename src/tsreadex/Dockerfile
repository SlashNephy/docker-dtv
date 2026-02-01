# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsreadex versioning=regex:^master-(?<patch>\d+)$
ARG TSREADEX_VERSION="master-240517"

ARG DEPENDENCIES="build-essential cmake"

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:49ba4b12ebea1d120da797a13e2cb480d74aae4c29c8d67f1646980b96060e92 AS download
ARG TSREADEX_VERSION

RUN git clone https://github.com/xtne6f/tsreadex -b "${TSREADEX_VERSION}" /app

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885 AS build
WORKDIR /app
ARG DEPENDENCIES

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=download /app/ ./
RUN cmake . && make

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885

COPY --from=build /app/tsreadex /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsreadex" ]
