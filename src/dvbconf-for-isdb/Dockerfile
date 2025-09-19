# syntax=docker/dockerfile:1

ARG REF="master"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.51.0@sha256:8b0f37fed1db4258b949599519e55e6261d9f7bb30cecb37dfee946d4005d422 AS download
ARG REF

ADD https://api.github.com/repos/Chinachu/dvbconf-for-isdb/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/Chinachu/dvbconf-for-isdb -b "${REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
