FROM postgres:17

VOLUME [ "/var/lib/postgresql/data" ]
EXPOSE 5432

ENV BACKEND_USER "backend"
ENV BACKEND_PASSWORD "password"
ENV READ_ONLY_USER "readonly"
ENV READ_ONLY_PASSWORD "password"
ENV DATABASE_NAME "backend"

COPY --chown=root:root infrastructure/docker/postgres/initialization /docker-entrypoint-initdb.d/

