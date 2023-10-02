# syntax=docker/dockerfile:1
ARG REF="master"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:3d7b0e24f9e1f72368f0e0a669b612b534a6a05c47f2c742346f7cec072f1590 AS downloader

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
