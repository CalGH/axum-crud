use deadpool_postgres::config::SslMode as deadpool_sslmode;
use deadpool_postgres::{Config as DeadpoolPgConfig, ManagerConfig, RecyclingMethod};
use tokio_postgres::config::SslMode as tokio_sslmode;
use tokio_postgres::Config as TokioPgConfig;

use dotenvy::dotenv;
use openssl::ssl::SslConnectorBuilder;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::env;
use std::str::FromStr;

macro_rules! log_pg_config {
    ($name:ident) => {
        dbg!(
            $name.get_user(),
            $name.get_hosts(),
            $name.get_dbname().unwrap(),
            $name.get_ports()
        );
    };
}

struct GeneralConfig {
    dp: Option<DeadpoolPgConfig>,
    tp: Option<TokioPgConfig>,
}

fn get_https_connector() -> MakeTlsConnector {
    let builder: SslConnectorBuilder = SslConnector::builder(SslMethod::tls()).unwrap();
    let connector: MakeTlsConnector = MakeTlsConnector::new(builder.build());
    connector
}

pub async fn get_tokio_client(pg_config: TokioPgConfig) -> tokio_postgres::Client {
    let connector = get_https_connector();

    let (client, connection) = match pg_config.connect(connector).await {
        Ok((client, connection)) => (client, connection),
        Err(err) => panic!("{}", err),
    };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

pub async fn get_deadpool_pool(deadpool_pgconfig: DeadpoolPgConfig) -> deadpool_postgres::Pool {
    let connector = get_https_connector();

    let pool = match deadpool_pgconfig.create_pool(connector) {
        Ok(pool) => pool,
        Err(err) => panic!("{}", err),
    };

    pool
}

fn log_config(confref: &GeneralConfig) {
    if let Some(conf) = &confref.dp {
        let details = conf.get_pg_config().unwrap();
        log_pg_config!(details);
    }
    if let Some(details) = &confref.tp {
        log_pg_config!(details);
    }
}

pub fn configure_tokio_postgres_db() -> TokioPgConfig {
    let connectstring = env::var("POSTGRES_CONNECTION_STRING").unwrap();

    let mut tokio_pgconfig = match TokioPgConfig::from_str(connectstring.as_str()) {
        Ok(conf) => GeneralConfig {
            dp: None,
            tp: Some(conf),
        },
        Err(err) => {
            panic!("invalid connection string var - {}", err)
        }
    };
    tokio_pgconfig
        .tp
        .as_mut()
        .unwrap()
        .ssl_mode(tokio_sslmode::Require);
    log_config(&tokio_pgconfig);
    tokio_pgconfig.tp.unwrap()
}

pub fn configure_deadpool_postgres_db() -> DeadpoolPgConfig {
    dotenv().expect("Must have .env");

    let mut deadpool_pgconfig = GeneralConfig {
        dp: Some(DeadpoolPgConfig::new()),
        tp: None,
    };

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let config = deadpool_pgconfig.dp.as_mut().unwrap();

    config.ssl_mode = Some(deadpool_sslmode::Require);
    config.user = Some(dotenvy::var("POSTGRES_USER").expect("POSTGRES_USER not set"));
    config.dbname = Some(dotenvy::var("POSTGRES_DBNAME").expect("POSTGRES_DBNAME not set"));
    config.hosts = Some(vec![
        dotenvy::var("POSTGRES_HOST").expect("POSTGRES_HOST not set")
    ]);
    config.ports = Some(vec![dotenvy::var("POSTGRES_PORT")
        .expect("POSTGRES_PORT not set")
        .parse()
        .unwrap()]);
    config.manager = Some(mgr_config);

    log_config(&deadpool_pgconfig);

    deadpool_pgconfig.dp.unwrap()
}
