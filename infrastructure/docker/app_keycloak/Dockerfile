FROM quay.io/keycloak/keycloak:26.6.4 AS builder

ENV KC_DB=postgres \
    KC_HEALTH_ENABLED=true \
    KC_METRICS_ENABLED=true

RUN /opt/keycloak/bin/kc.sh build

FROM quay.io/keycloak/keycloak:26.6.4

COPY --from=builder /opt/keycloak/ /opt/keycloak/
COPY --from=localhost/tmp_health_checker:latest /http_health_checker /bin/http_health_checker

USER 1000

# OIDC endpoints ; health and metrics ; Infinispan clustering
EXPOSE 8080 9000 7800

HEALTHCHECK \
    --start-period=60s \
    --interval=10s \
    --timeout=10s \
    --retries=3 \
    CMD [ "/bin/http_health_checker", "http://127.0.0.1:9000/health/ready" ]
