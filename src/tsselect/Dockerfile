# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsselect_gcc
ARG TSSELECT_VERSION="r3"

ARG DEPENDENCIES="build-essential"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.42.0@sha256:2f4cc0850fe005ffb3e91a48f92bd941acb2888dff910d1f21bb98fdbe3e0876 AS downloader
ARG TSSELECT_VERSION

RUN git clone https://github.com/xtne6f/tsselect_gcc -b "${TSSELECT_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:4f5d8d928d9398c9eae7b6512848b9ae6022006a74f816c2fe2cbf8e598ae242 AS builder
ARG DEPENDENCIES

COPY --from=downloader /app/ /app/
WORKDIR /app/src
RUN <<EOF
  apt update
  apt install -y --no-install-recommends ${DEPENDENCIES}

  make
EOF

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:4f5d8d928d9398c9eae7b6512848b9ae6022006a74f816c2fe2cbf8e598ae242
COPY --from=build /app/src/tsselect /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsselect" ]
