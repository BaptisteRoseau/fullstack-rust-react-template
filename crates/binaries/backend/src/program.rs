use std::sync::{Arc, RwLock};

use anyhow::Error;
use api::AppState;
use api::routes::{public_routes, try_metrics_routes};
use axum::Router;
use axum_prometheus::PrometheusMetricLayerBuilder;
use config::Config;
use database::backends::Postgres;
use storage::backends::S3;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::task::JoinHandle;
use tracing::{info, warn};

pub(crate) async fn run(config: &Config) -> Result<(), anyhow::Error> {
    logging::init_logger(config.debug);

    /* ===========================
    * APP STATE
    =========================== */

    info!("Initializing Database...");
    let database = Postgres::try_from(config).await?;
    info!("Initialized Database");

    info!("Initializing Storage...");
    let storage = S3::try_from(config)?;
    info!("Initialized Storage");

    let state = AppState::new(
        Arc::new(RwLock::new(database)),
        Arc::new(RwLock::new(storage)),
    );

    /* ===========================
    * SERVERS
    =========================== */

    let mut servers = vec![];

    // PUBLIC ROUTES
    info!("Initializing public API router...");
    let mut public_routes = public_routes(config, state);

    // PROMETHEUS
    if let Some(prometheus_config) = &config.prometheus {
        info!("Initializing Metrics handler...");
        let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
            .with_prefix("api")
            .with_default_metrics()
            .build_pair();

        public_routes = public_routes.layer(prometheus_layer);
        info!("Initialized Metrics handler");

        info!("Initializing Prometheus metrics endpoint...");
        let metrics_routes = try_metrics_routes(&prometheus_config.path, metric_handle)?;

        if (prometheus_config.ip == config.server.ip
            && prometheus_config.port == config.server.port)
        {
            warn!("Merging Prometheus metrics endpoint with public API");
            public_routes = public_routes.merge(metrics_routes);
        } else {
            info!(
                "Binding separate Prometheus metrics endpoint onto {}:{}...",
                prometheus_config.ip, prometheus_config.port
            );
            let prometheus_metrics_server = tokio::spawn(serve_onto(
                (prometheus_config.ip, prometheus_config.port),
                metrics_routes,
            ));
            servers.push(prometheus_metrics_server);
        }

        info!(
            "Initialized Prometheus metrics endpoint on {}",
            &prometheus_config.path
        );
    }

    // Binding public routes at the end to make sure metric layer is added
    info!(
        "Binding public API onto {}:{}...",
        config.server.ip, config.server.port
    );
    let public_server = tokio::spawn(serve_onto(
        (config.server.ip, config.server.port),
        public_routes,
    ));
    info!("Initialized public API router");

    servers.push(public_server);
    set_shutdown_signal(&mut servers).await;

    info!("Ready to receive requests");
    for server in servers {
        let _ = tokio::join!(server);
    }

    info!("Successfully shut down the server. Bye bye!");
    Ok(())
}

async fn serve_onto<A>(address: A, routes: Router) -> Result<(), anyhow::Error>
where
    A: ToSocketAddrs,
{
    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, routes).await?;
    Ok(())
}

// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn set_shutdown_signal(servers: &mut Vec<JoinHandle<Result<(), Error>>>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install sigterm signal handler")
            .recv()
            .await;
    };

    #[cfg(unix)]
    let interrupt = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
            .expect("failed to install sigint signal handler")
            .recv()
            .await;
    };

    #[cfg(unix)]
    let quit = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::quit())
            .expect("failed to install sigquit signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            info!("Received SIGINT, existing...");
            for server in servers{
                server.abort();
            }
        },
        () = terminate => {
            info!("Received SIGTERM, existing...");
            for server in servers{
                server.abort();
            }
        },
        () = interrupt => {
            info!("Received SIGINT, existing...");
            for server in servers{
                server.abort();
            }
        },
        () = quit => {
            info!("Received SIGQUIT, existing...");
            for server in servers{
                server.abort();
            }
        },
    }
}
