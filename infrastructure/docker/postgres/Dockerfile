FROM postgres:18

ENV PGDATA "/var/lib/postgresql/data" 
VOLUME [ "/var/lib/postgresql/data" ]
EXPOSE 5432

COPY --chown=root:root infrastructure/docker/postgres/initialization /docker-entrypoint-initdb.d/

