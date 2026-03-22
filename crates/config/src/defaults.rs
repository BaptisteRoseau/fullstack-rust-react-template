use std::net::{IpAddr, Ipv4Addr};

pub(crate) const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub(crate) const DEFAULT_PORT: u16 = 6969;

pub(crate) const DEFAULT_SWAGGER_IP: IpAddr = LOCALHOST;
pub(crate) const DEFAULT_SWAGGER_PORT: u16 = 7070;

pub(crate) const DEFAULT_SWAGGER_UI_PATH: &str = "/v1/docs/swagger-ui";
pub(crate) const DEFAULT_OPENAPI_PATH: &str = "/v1/docs/openapi.json";

pub(crate) const DEFAULT_PROMETHEUS_IP: IpAddr = LOCALHOST;
pub(crate) const DEFAULT_PROMETHEUS_PORT: u16 = 9100;

pub(crate) const DEFAULT_DATABASE_HOST: &str = "127.0.0.1";
pub(crate) const DEFAULT_DATABASE_PORT: u16 = 5432;
pub(crate) const DEFAULT_DATABASE_NAME: &str = "backend";
pub(crate) const DEFAULT_DATABASE_USER: &str = "backend";
pub(crate) const DEFAULT_DATABASE_PASSWORD: &str = "password";

pub(crate) const DEFAULT_S3_HOST: &str = "127.0.0.1";
pub(crate) const DEFAULT_S3_PORT: u16 = 9000;
pub(crate) const DEFAULT_S3_USER: &str = "backend";
pub(crate) const DEFAULT_S3_PASSWORD: &str = "password";

pub(crate) const DEFAULT_API_TIMEOUT_SEC: u16 = 20;

pub(crate) const DEFAULT_CONFIG_FILE_PATH: &str = ".config.yaml";
