# syntax=docker/dockerfile:1
ARG REF="master"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:fd865da5f887f2fdb7af19c3a33fe9c98cb00a26859df66508cadadc35cb8676 AS downloader

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
