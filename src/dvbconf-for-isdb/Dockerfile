# syntax=docker/dockerfile:1

# renovate: datasource=git-refs depName=https://github.com/Chinachu/dvbconf-for-isdb branch=master
ARG DVBCONF_FOR_ISDB_REF="5a40d99185703b90e8d15b00894cef2f64442155"

FROM --platform=$BUILDPLATFORM alpine/git:v2.54.0@sha256:6a1fdb962814ca11508ca8d1bab94375b5b7ac4e4a9a40981c29fc32c02f095d AS download
ARG DVBCONF_FOR_ISDB_REF

RUN git clone https://github.com/Chinachu/dvbconf-for-isdb --revision "${DVBCONF_FOR_ISDB_REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
