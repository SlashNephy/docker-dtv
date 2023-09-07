# syntax=docker/dockerfile:1
ARG REF="master"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:6662bb5ae28ce4ce754cb4cc4b03deb9b4f24ab052b812b966aa773942e27745 AS downloader

RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends git ca-certificates
EOF

FROM --platform=$BUILDPLATFORM downloader AS download
ARG REF

ADD https://api.github.com/repos/Chinachu/dvbconf-for-isdb/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/Chinachu/dvbconf-for-isdb -b "${REF}" --recursive /app

FROM scratch
COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
