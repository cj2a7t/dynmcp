use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Error, Result};
use mcp_common::{
    cache::mcp_cache::McpCache,
    provider::global_provider::{get_app_config, init_etcd_global, init_mysql_pool},
};

use crate::datasource::{
    datasource::DataSource, ds_enum::DataSourceEnum, etcd_ds::EtcdDataSource,
    mysql_ds::MysqlDataSource,
};

pub enum DataSourceFactory {
    Etcd,
    Mysql,
}

impl FromStr for DataSourceFactory {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "etcd" => Ok(DataSourceFactory::Etcd),
            "mysql" => Ok(DataSourceFactory::Mysql),
            _ => Err(anyhow!("Unknown data source type: {}", s)),
        }
    }
}

impl DataSourceFactory {
    pub async fn build(mcp_cache: Arc<McpCache>) -> Result<Arc<DataSourceEnum>> {
        let config: Arc<mcp_common::config::config::AppConfig> = get_app_config()?;
        let factory: DataSourceFactory = DataSourceFactory::from_str(&config.app.data_source)?;
        match factory {
            DataSourceFactory::Etcd => {
                let etcd_config = config
                    .data_source
                    .etcd
                    .as_ref()
                    .ok_or_else(|| anyhow!("Etcd config not found"))?;

                init_etcd_global(
                    etcd_config.endpoints.clone(),
                    etcd_config.username.clone(),
                    etcd_config.password.clone(),
                )
                .await?;

                let ds = Arc::new(EtcdDataSource::new(mcp_cache));
                let ds_clone = ds.clone();
                tokio::spawn(async move {
                    if let Err(e) = ds_clone.fetch_and_watch().await {
                        tracing::error!("ETCD fetch_and_watch failed: {:?}", e);
                    }
                });
                Ok(Arc::new(DataSourceEnum::Etcd(ds)))
            }
            DataSourceFactory::Mysql => {
                let mysql_config = config
                    .data_source
                    .mysql
                    .as_ref()
                    .ok_or_else(|| anyhow!("MySQL config not found"))?;

                init_mysql_pool(&mysql_config.url).await?;

                let ds = Arc::new(MysqlDataSource::new(mcp_cache));
                let ds_clone = ds.clone();
                tokio::spawn(async move {
                    if let Err(e) = ds_clone.fetch_and_watch().await {
                        tracing::error!("MySQL fetch_and_watch failed: {:?}", e);
                    }
                });
                Ok(Arc::new(DataSourceEnum::Mysql(ds)))
            }
        }
    }
}
