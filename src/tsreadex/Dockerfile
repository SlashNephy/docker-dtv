FROM --platform=$BUILDPLATFORM debian:11.7-slim@sha256:3460d74bec6b88496cd183d7731930be55234c094f581f7dbdd96f56c1fc34d8 AS git
ADD https://api.github.com/repos/xtne6f/tsreadex/git/refs/heads/master /tmp/git.json
RUN apt update \
    && apt install -y --no-install-recommends git ca-certificates \
    && git clone https://github.com/xtne6f/tsreadex /app

# Build stage
FROM --platform=$TARGETPLATFORM debian:11.7-slim@sha256:3460d74bec6b88496cd183d7731930be55234c094f581f7dbdd96f56c1fc34d8 AS build
WORKDIR /app
COPY --from=git /app/ /app/
RUN apt update \
    && apt install -y --no-install-recommends \
        build-essential \
        git \
        cmake \
    && cmake . \
    && make

# Final stage
FROM --platform=$TARGETPLATFORM debian:11.7-slim@sha256:3460d74bec6b88496cd183d7731930be55234c094f581f7dbdd96f56c1fc34d8 AS runtime
COPY --from=build /app/tsreadex /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsreadex" ]
