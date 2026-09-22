FROM nxyt/sqlx-cli:v0.70

USER 10001
WORKDIR /
ENTRYPOINT [ "/bin/sqlx", "migrate", "run", "--source", "/migrations" ]

# Last to cache previous layers
COPY crates/database/migrations/ /migrations/
