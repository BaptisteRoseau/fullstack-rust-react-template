use database::backends::Postgres;
use sqlx::PgPool;
use testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner};
use testcontainers_modules::postgres::Postgres as PgImage;

/// Pure-SQL implementation of uuidv7() for use in tests.
/// The production image provides this via an extension; here we install
/// a compatible pure-SQL version so the migrations can run unmodified.
const UUIDV7_INIT_SQL: &str = r#"
CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE OR REPLACE FUNCTION uuidv7() RETURNS uuid AS $$
  -- UUIDv7: 48-bit unix_ms | 4-bit version(7) | 12-bit seq | 2-bit variant | 62-bit random
  SELECT encode(
    overlay(
      overlay(
        gen_random_bytes(16)
        placing substring(int8send((extract(epoch from clock_timestamp())*1000)::bigint) from 3)
        from 1 for 6
      )
      placing '\x7000'::bytea  -- version nibble = 7, clear lower 4 bits of byte 6
      from 7 for 2
    ),
    'hex'
  )::uuid
$$ LANGUAGE SQL;
"#;

pub struct PostgresFixture {
    _container: ContainerAsync<PgImage>,
    pub connection_string: String,
}

impl PostgresFixture {
    pub async fn start() -> Self {
        let container = PgImage::default()
            .with_init_sql(UUIDV7_INIT_SQL.as_bytes().to_vec())
            .with_env_var("LANG", "en_US.utf8")
            .start()
            .await
            .expect("failed to start postgres container");
        let port = container.get_host_port_ipv4(5432).await.unwrap();
        let connection_string =
            format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");

        let pool = PgPool::connect(&connection_string)
            .await
            .expect("failed to connect for migrations");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("migrations failed");

        Self {
            _container: container,
            connection_string,
        }
    }

    pub async fn make_postgres(&self) -> Postgres {
        Postgres::try_from_url(&self.connection_string)
            .await
            .expect("failed to create Postgres client")
    }
}
