FROM alpine:3.18.4@sha256:eece025e432126ce23f223450a0326fbebde39cdf496a85d8c016293fc851978

# install samba
RUN apk update \
    && apk add samba rsyslog tzdata

RUN mkdir -p /mnt

ARG SMB_USER
ARG SMB_PASSWORD

RUN adduser -S $SMB_USER
RUN echo $SMB_PASSWORD | tee - | smbpasswd -a -s $SMB_USER

RUN { \
    echo "[global]"; \
    echo "  security = user"; \
    echo "  map to guest = Bad User"; \
    echo "  server min protocol = SMB2"; \
    echo "  log level = 1"; \
    echo "  log file = /var/log/samba/%I.log"; \
    echo "  max log size = 4096"; \
    echo ""; \
    echo "[mnt]"; \
    echo "  path = /mnt"; \
    echo "  valid users = ${SMB_USER}"; \
    echo "  guest ok = no"; \
    echo "  browsable = yes"; \
    echo "  writable = no"; \
    echo "  vfs objects = full_audit"; \
    echo "  full_audit:prefix = %m|%I"; \
    echo "  full_audit:success = open"; \
    echo "  full_audit:failure = connect"; \
    echo "  full_audit:facility = local1"; \
} > /etc/samba/smb.conf

RUN echo "local1.*    /var/log/samba/audit.log" >> /etc/rsyslog.conf
RUN echo "local1.*    /dev/stdout" >> /etc/rsyslog.conf

# start smbd as foreground
ENTRYPOINT ["/bin/ash", "-c", "rsyslogd -f /etc/rsyslog.conf -n & smbd -FS --no-process-group --log-stdout"]
