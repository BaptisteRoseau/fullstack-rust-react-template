FROM ghcr.io/gethomepage/homepage:v1.4.0

COPY --from=localhost/tmp_health_checker:latest /http_health_checker /bin/http_health_checker

RUN mkdir -p /app/config/logs && \
    chown -R 1000:1000 /app/config

VOLUME [ "/app/config" ]

EXPOSE 3000
HEALTHCHECK \
    --start-period=10s \
    --interval=10s \
    --timeout=10s \
    --retries=3 \
    CMD [ "/bin/http_health_checker", "http://127.0.0.1:3000/" ]
