# syntax=docker/dockerfile:1

# renovate: datasource=git-refs depName=https://github.com/Chinachu/dvbconf-for-isdb branch=master
ARG DVBCONF_FOR_ISDB_REF="e337342d140f24d8d22e1ab75d72a475192dd6ba"

FROM --platform=$BUILDPLATFORM alpine/git:v2.54.0@sha256:ae0f6f4bce38d2b8c40becc0d6241a08d9f57186ea03029de2189a2b5e722d94 AS download
ARG DVBCONF_FOR_ISDB_REF

RUN git clone https://github.com/Chinachu/dvbconf-for-isdb --revision "${DVBCONF_FOR_ISDB_REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
