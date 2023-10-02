FROM alpine:3.18@sha256:eece025e432126ce23f223450a0326fbebde39cdf496a85d8c016293fc851978

RUN apk --no-cache add python3 python3-dev bash build-base linux-headers \
    && pip3 install --upgrade pip \
    && pip3 install aiohttp psutil discord.py

COPY app.py /app.py
WORKDIR /

ENTRYPOINT ["python3", "/app.py"]
