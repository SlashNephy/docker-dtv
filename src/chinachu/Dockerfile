# syntax=docker/dockerfile:1

# renovate: datasource=git-refs depName=https://github.com/Chinachu/Chinachu branch=gamma
ARG CHINACHU_REF="6b49bd77bb733c2843571cea1785719ca0819dae"

ARG RUNTIMES="curl"
ARG DEPENDENCIES="build-essential git sudo ca-certificates python3 wget xz-utils"

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:d46d88ab234733c6b6a9771acd6d1384172fd0e2224e0232bdae32ec671aa099 AS download
ARG CHINACHU_REF

RUN git clone https://github.com/Chinachu/Chinachu --revision "${CHINACHU_REF}" --recursive /app

FROM --platform=$TARGETPLATFORM node:16.20.2-bookworm-slim@sha256:89061680359f5d9103d24f0066e2c96a8018c24d5c4fe8eb8d59d16dd4ab64ba
WORKDIR /app
ARG RUNTIMES
ARG DEPENDENCIES
ENV DOCKER="YES"

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES} ${DEPENDENCIES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=download /app/ ./
RUN mkdir -p log bin && chown -R node .

USER node
SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  echo 2 | ./chinachu installer
  echo 4 | ./chinachu installer
  echo 5 | ./chinachu installer

  ./chinachu service operator initscript | tee bin/chinachu-operator
  ./chinachu service wui initscript | tee bin/chinachu-wui
EOF

USER root
SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  mv bin/chinachu-operator bin/chinachu-wui /usr/local/bin/
  rmdir bin
  chmod u+x /usr/local/bin/chinachu-operator /usr/local/bin/chinachu-wui
EOF

RUN SUDO_FORCE_REMOVE=yes apt-get purge -y --auto-remove ${DEPENDENCIES}

COPY ./entrypoint.sh /

USER node
CMD ["/entrypoint.sh"]
