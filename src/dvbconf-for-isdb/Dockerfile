# syntax=docker/dockerfile:1

# renovate: datasource=git-refs depName=https://github.com/Chinachu/dvbconf-for-isdb branch=master
ARG DVBCONF_FOR_ISDB_REF="e337342d140f24d8d22e1ab75d72a475192dd6ba"

FROM --platform=$BUILDPLATFORM alpine/git:v2.54.0@sha256:6f3b5029566da8e90b24945933dcd806be866b64b1e706f51828bf84faccf21b AS download
ARG DVBCONF_FOR_ISDB_REF

RUN git clone https://github.com/Chinachu/dvbconf-for-isdb --revision "${DVBCONF_FOR_ISDB_REF}" /app

FROM scratch

COPY --from=download /app/conf/ /usr/local/dvbconf-for-isdb/
