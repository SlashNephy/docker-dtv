# syntax=docker/dockerfile:1

# renovate: datasource=git-refs depName=https://github.com/Chinachu/dvbconf-for-isdb branch=master
ARG DVBCONF_FOR_ISDB_REF="e337342d140f24d8d22e1ab75d72a475192dd6ba"

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:3b7890cb947afd3f71adab55aa6b549ad0f8ddc4b9ed28b027563d73d49d8e11 AS download
ARG DVBCONF_FOR_ISDB_REF

RUN git clone https://github.com/Chinachu/dvbconf-for-isdb --revision "${DVBCONF_FOR_ISDB_REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
