# syntax=docker/dockerfile:1

ARG REF="master"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.49.0@sha256:a6a388afc0e8000b0ad753010a546ded487263e5923007417055f5c84018719a AS download
ARG REF

ADD https://api.github.com/repos/Chinachu/dvbconf-for-isdb/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/Chinachu/dvbconf-for-isdb -b "${REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
