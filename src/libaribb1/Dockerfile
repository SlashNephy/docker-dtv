FROM --platform=$BUILDPLATFORM debian:12.2-slim@sha256:b55e2651b71408015f8068dd74e1d04404a8fa607dd2cfe284b4824c11f4d9bd AS git
ADD https://api.github.com/repos/stz2012/libarib25/git/refs/heads/master /tmp/git.json
RUN apt update \
    && apt install -y --no-install-recommends git ca-certificates \
    && git clone https://github.com/stz2012/libarib25 /app

FROM --platform=$BUILDPLATFORM debian:12.2-slim@sha256:b55e2651b71408015f8068dd74e1d04404a8fa607dd2cfe284b4824c11f4d9bd AS source
WORKDIR /app
COPY --from=git /app/ /app/
COPY ./*.patch /tmp/
RUN apt update \
    && apt install -y --no-install-recommends git ca-certificates \
    && git apply --numstat --summary --check --apply --ignore-whitespace -v /tmp/*.patch

# Build libaribb1
FROM --platform=$TARGETPLATFORM debian:12.2-slim@sha256:b55e2651b71408015f8068dd74e1d04404a8fa607dd2cfe284b4824c11f4d9bd AS build
WORKDIR /app
COPY --from=source /app/ /app/
RUN apt update \
    && apt install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        cmake \
        pkg-config \
        libpcsclite-dev \
    && cmake -DCMAKE_VERBOSE_MAKEFILE=TRUE . \
    && make \
    && make install \
    && mv /usr/local/bin/b25 /usr/local/bin/b1 \
    && mv /usr/local/lib/libarib25.a /usr/local/lib/libarib1.a \
    && mv /usr/local/lib/libarib25.so /usr/local/lib/libarib1.so \
    && mv /usr/local/lib/libarib25.so.0 /usr/local/lib/libarib1.so.0 \
    && mv /usr/local/lib/libarib25.so.0.2.5 /usr/local/lib/libarib1.so.0.2.5 \
    && mv /usr/local/include/arib25 /usr/local/include/arib1 \
    && mv /usr/local/lib/pkgconfig/libarib25.pc /usr/local/lib/pkgconfig/libarib1.pc

FROM --platform=$TARGETPLATFORM debian:12.2-slim@sha256:b55e2651b71408015f8068dd74e1d04404a8fa607dd2cfe284b4824c11f4d9bd AS runtime

COPY ./b1-stream /usr/local/bin/
RUN chmod +x /usr/local/bin/b1-stream

COPY --from=build /usr/local/include/ /usr/local/include/
COPY --from=build /usr/local/bin/ /usr/local/bin/
COPY --from=build /usr/local/lib/ /usr/local/lib/

ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib
ENTRYPOINT [ "/usr/local/bin/b1" ]
