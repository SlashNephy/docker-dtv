# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=stz2012/libarib25
ARG LIBARIB25_VERSION="v0.2.5-20220902"

ARG DEPENDENCIES="build-essential ca-certificates cmake pkg-config libpcsclite-dev"

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:d86f367afb53d022acc4377741e7334bc20add161bb10234272b91b459b4b7d8 AS source
ARG LIBARIB25_VERSION

RUN git clone https://github.com/stz2012/libarib25 -b "${LIBARIB25_VERSION}" /app

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:d86f367afb53d022acc4377741e7334bc20add161bb10234272b91b459b4b7d8 AS patch-source
WORKDIR /app

COPY --from=source /app/ ./
COPY ./*.patch /tmp/

RUN git apply --numstat --summary --check --apply --ignore-whitespace -v /tmp/*.patch

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

COPY --from=patch-source /app/ ./

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  cmake -DCMAKE_VERBOSE_MAKEFILE=TRUE .
  make
  make install

  mv /usr/local/bin/b25 /usr/local/bin/b1
  mv /usr/local/lib/libarib25.a /usr/local/lib/libarib1.a
  mv /usr/local/lib/libarib25.so /usr/local/lib/libarib1.so
  mv /usr/local/lib/libarib25.so.0 /usr/local/lib/libarib1.so.0
  mv /usr/local/lib/libarib25.so.0.2.5 /usr/local/lib/libarib1.so.0.2.5
  mv /usr/local/include/arib25 /usr/local/include/arib1
  mv /usr/local/lib/pkgconfig/libarib25.pc /usr/local/lib/pkgconfig/libarib1.pc
EOF

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885
ENV LD_LIBRARY_PATH="${LD_LIBRARY_PATH}:/usr/local/lib"

COPY ./b1-stream /usr/local/bin/
RUN chmod +x /usr/local/bin/b1-stream

COPY --from=build /usr/local/include/ /usr/local/include/
COPY --from=build /usr/local/bin/ /usr/local/bin/
COPY --from=build /usr/local/lib/ /usr/local/lib/

ENTRYPOINT [ "/usr/local/bin/b1" ]
