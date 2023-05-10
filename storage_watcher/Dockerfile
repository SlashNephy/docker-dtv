FROM alpine:3.18@sha256:02bb6f428431fbc2809c5d1b41eab5a68350194fb508869a33cb1af4444c9b11

RUN apk --no-cache add python3 python3-dev bash build-base linux-headers \
    && pip3 install --upgrade pip \
    && pip3 install aiohttp psutil discord.py

COPY app.py /app.py
WORKDIR /

ENTRYPOINT ["python3", "/app.py"]
