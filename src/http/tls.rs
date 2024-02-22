use native_tls::TlsConnector as NativeTlsConnector;
use tokio_native_tls::TlsConnector;

pub struct TlsConnectorBuilder {
    pub connector: TlsConnector,
}

impl TlsConnectorBuilder {
    pub fn new() -> Self {
        Self {
            connector: TlsConnector::from(match NativeTlsConnector::builder().build() {
                Ok(tls_connector) => tls_connector,
                Err(why) => panic!("tls connector error: {:?}", why),
            }),
        }
    }
}
