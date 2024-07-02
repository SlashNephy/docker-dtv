# syntax=docker/dockerfile:1

ARG REF="master"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.45.2@sha256:46f0271bedcf85236982a3672357a3c62ed75aee86559f722f158d1657ec0d7d AS download
ARG REF

ADD https://api.github.com/repos/Chinachu/dvbconf-for-isdb/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/Chinachu/dvbconf-for-isdb -b "${REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
