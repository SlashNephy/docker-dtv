# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=stz2012/libarib25
ARG LIBARIB25_VERSION="v0.2.5-20220902"

ARG DEPENDENCIES="build-essential ca-certificates cmake pkg-config libpcsclite-dev"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.44.0@sha256:b91c3ac4915cbf6ba813175a0387bca69a70b52e1b57a22e212322036c47da23 AS source
ARG LIBARIB25_VERSION

RUN git clone https://github.com/stz2012/libarib25 -b "${LIBARIB25_VERSION}" /app

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.44.0@sha256:b91c3ac4915cbf6ba813175a0387bca69a70b52e1b57a22e212322036c47da23 AS patch-source
WORKDIR /app

COPY --from=source /app/ ./
COPY ./*.patch /tmp/

RUN git apply --numstat --summary --check --apply --ignore-whitespace -v /tmp/*.patch

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:dcf7318b55ff446622951f631b69fbee828d922e67223bd2ddbf8f692b1bcf83 AS build
WORKDIR /app
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=patch-source /app/ ./
RUN <<EOF
  set -eux

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

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:dcf7318b55ff446622951f631b69fbee828d922e67223bd2ddbf8f692b1bcf83
ENV LD_LIBRARY_PATH="${LD_LIBRARY_PATH}:/usr/local/lib"

COPY ./b1-stream /usr/local/bin/
RUN chmod +x /usr/local/bin/b1-stream

COPY --from=build /usr/local/include/ /usr/local/include/
COPY --from=build /usr/local/bin/ /usr/local/bin/
COPY --from=build /usr/local/lib/ /usr/local/lib/

ENTRYPOINT [ "/usr/local/bin/b1" ]
