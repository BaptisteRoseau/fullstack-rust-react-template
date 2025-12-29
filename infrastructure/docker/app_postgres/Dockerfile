FROM postgres:18

ENV PGDATA="/var/lib/postgresql/data" 
VOLUME [ "/var/lib/postgresql/data" ]
EXPOSE 5432
HEALTHCHECK --interval=10s --timeout=5s --start-period=5s --retries=3 \
  CMD /bin/healthcheck.sh

COPY --chown=root:root infrastructure/docker/postgres/initialization /docker-entrypoint-initdb.d/
COPY --chown=root:root infrastructure/docker/postgres/healthcheck.sh /bin/healthcheck.sh
