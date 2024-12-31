# syntax=docker/dockerfile:1

ARG REF="master"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.47.1@sha256:87734424c49f37eb50d9d7000b88ab66aff8ca05a21dc66266117589a8a55aae AS download
ARG REF

ADD https://api.github.com/repos/Chinachu/dvbconf-for-isdb/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/Chinachu/dvbconf-for-isdb -b "${REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
