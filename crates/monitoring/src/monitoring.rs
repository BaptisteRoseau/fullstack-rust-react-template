pub trait MonitoringConnectionName {
    fn name() -> &'static str;
}

pub trait OnConnection<T: MonitoringConnectionName> {
    fn on_connection_success(&self) {
        // Prometheus connection UP
    }
    fn on_connection_failure(&self) {
        // Prometheus metric connection DOWN
    }
}
