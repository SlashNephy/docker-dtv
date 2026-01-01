# syntax=docker/dockerfile:1

# renovate: datasource=git-refs depName=Chinachu/dvbconf-for-isdb branch=master
ARG DVBCONF_FOR_ISDB_REF="e337342d140f24d8d22e1ab75d72a475192dd6ba"

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:d86f367afb53d022acc4377741e7334bc20add161bb10234272b91b459b4b7d8 AS download
ARG DVBCONF_FOR_ISDB_REF

RUN git clone https://github.com/Chinachu/dvbconf-for-isdb --revision "${DVBCONF_FOR_ISDB_REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
