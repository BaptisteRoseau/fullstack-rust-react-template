use metrics::gauge;

const CONNECTION_UP: u8 = 0;
const CONNECTION_DOWN: u8 = 1;

pub trait MonitoringConnectionName {
    fn monitoring_connection_name(&self) -> String;
}

pub trait OnConnection<T: MonitoringConnectionName> {
    fn on_connection_success(item: &T) {
        gauge!(item.monitoring_connection_name()).set(CONNECTION_UP);
    }

    fn on_connection_failure(item: &T) {
        gauge!(item.monitoring_connection_name()).set(CONNECTION_DOWN);
    }
}
