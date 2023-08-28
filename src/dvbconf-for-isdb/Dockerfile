# syntax=docker/dockerfile:1
ARG REF="master"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:9745edf9e4904fd9ff5fbb2e0c0ba0c28a74e77ca645ed751a0aadff56c439bc AS downloader

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
