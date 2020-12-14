FROM alpine:3.12.2

# install samba
RUN apk update \
    && apk add samba rsyslog

RUN mkdir -p /mnt

ARG SMB_USER
ARG SMB_PASSWORD

RUN adduser -S $SMB_USER
RUN echo $SMB_PASSWORD | tee - | smbpasswd -a -s $SMB_USER

RUN { \
    echo "[global]"; \
    echo "  security = user"; \
    echo "  min protocol = SMB3"; \
    echo "  log level = 2"; \
    echo "  log file = /var/log/samba/%m.log"; \
    echo "  max log size = 4096"; \
    echo ""; \
    echo "[mnt]"; \
    echo "  path = /mnt"; \
    echo "  valid users = ${SMB_USER}"; \
    echo "  guest ok = no"; \
    echo "  browsable = yes"; \
    echo "  writable = no"; \
    echo "  vfs objects = full_audit"; \
    echo "  full_audit:prefix = %m|%I|%S"; \
    echo "  full_audit:success = read"; \
    echo "  full_audit:facility = local1"; \
    echo "  full_audit:priority = info"; \
} > /etc/samba/smb.conf

RUN echo "local1.*    /var/log/samba/audit.log" > /etc/rsyslog.conf

# start smbd as foreground
ENTRYPOINT ["/bin/ash", "-c", "rsyslogd -f /etc/rsyslog.conf -n & smbd -FS --no-process-group"]
