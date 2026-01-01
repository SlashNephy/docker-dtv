# syntax=docker/dockerfile:1

ARG REF="master"

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:d86f367afb53d022acc4377741e7334bc20add161bb10234272b91b459b4b7d8 AS download
ARG REF

ADD https://api.github.com/repos/Chinachu/dvbconf-for-isdb/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/Chinachu/dvbconf-for-isdb -b "${REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
