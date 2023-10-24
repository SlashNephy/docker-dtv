FROM --platform=$BUILDPLATFORM debian:12.2-slim@sha256:b55e2651b71408015f8068dd74e1d04404a8fa607dd2cfe284b4824c11f4d9bd AS git
ADD https://api.github.com/repos/l3tnun/EPGStation/git/refs/heads/v2 /tmp/EPGStation.json
RUN apt update \
    && apt install -y --no-install-recommends git ca-certificates \
    && git clone -b v2 https://github.com/l3tnun/EPGStation /app

FROM --platform=$BUILDPLATFORM debian:12.2-slim@sha256:b55e2651b71408015f8068dd74e1d04404a8fa607dd2cfe284b4824c11f4d9bd AS source
WORKDIR /app
COPY --from=git /app/ /app/
COPY ./*.patch /tmp/
RUN apt update \
    && apt install -y --no-install-recommends git ca-certificates \
    && git apply --numstat --summary --check --apply --ignore-whitespace -v /tmp/*.patch

FROM --platform=$BUILDPLATFORM debian:12.2-slim@sha256:b55e2651b71408015f8068dd74e1d04404a8fa607dd2cfe284b4824c11f4d9bd AS version
ARG COMMIT_ID=unknown
COPY --from=git /app/package.json /app/package.json
COPY --from=git /app/client/package.json /app/client/package.json
SHELL ["/bin/bash", "-c"]
RUN apt update \
    && apt install -y --no-install-recommends jq \
    && cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime \
    && jq ".version=\"`jq -r .version /app/package.json`-`date '+%Y%m%d-%H%M%S'`-${COMMIT_ID:0:7}\"" /app/package.json > /tmp/server.package.json \
    && jq ".version=\"`jq -r .version /app/client/package.json`-`date '+%Y%m%d-%H%M%S'`-${COMMIT_ID:0:7}\"" /app/client/package.json > /tmp/client.package.json \
    && mv -f /tmp/server.package.json /app/package.json \
    && mv -f /tmp/client.package.json /app/client/package.json

FROM --platform=$BUILDPLATFORM node:20.8.1-bullseye-slim@sha256:682c1557c5a8cd6f8a78db3bd315ed968b3a854de2a16c2b8ce713cc92152062 AS build-client
COPY --from=git /app/client/package*.json /app/client/
WORKDIR /app/client
RUN npm install
COPY --from=source /app /app/
RUN npm run build

FROM --platform=$TARGETPLATFORM node:20.8.1-bullseye-slim@sha256:682c1557c5a8cd6f8a78db3bd315ed968b3a854de2a16c2b8ce713cc92152062 AS build-server
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update \
    && apt install -y build-essential python
WORKDIR /app
COPY --from=git /app/package*.json /app/
ENV DOCKER="YES"
RUN npm install --no-save
COPY --from=source /app .
RUN rm -rf client
RUN npm run build-server

FROM --platform=$TARGETPLATFORM node:20.8.1-bullseye-slim@sha256:682c1557c5a8cd6f8a78db3bd315ed968b3a854de2a16c2b8ce713cc92152062 AS runtime
COPY --from=build-server /app/ /app/
COPY --from=build-client /app/client/ /app/client/
COPY --from=version /app/package.json /app/package.json
COPY --from=version /app/client/package.json /app/client/package.json

WORKDIR /app
ENTRYPOINT [ "node", "dist/index.js" ]
