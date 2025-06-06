# syntax=docker/dockerfile:1

ARG REF="master"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.49.0@sha256:7084776e6388614b2919779a7612d284804da1c63ea54b1f378446b8a2a2a4b9 AS download
ARG REF

ADD https://api.github.com/repos/Chinachu/dvbconf-for-isdb/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/Chinachu/dvbconf-for-isdb -b "${REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
