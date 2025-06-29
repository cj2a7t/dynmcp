use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use mcp_common::{
    cache::mcp_cache::McpCache,
    provider::global_provider::{init_etcd_global, init_mysql_pool},
    DynMcpArgs,
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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "etcd" => Ok(DataSourceFactory::Etcd),
            "mysql" => Ok(DataSourceFactory::Mysql),
            _ => Err(anyhow::anyhow!("Unknown data source type: {}", s)),
        }
    }
}

impl DataSourceFactory {
    pub async fn create_data_source_enum(
        &self,
        args: DynMcpArgs,
        mcp_cache: Arc<McpCache>,
    ) -> Result<Arc<DataSourceEnum>> {
        match self {
            DataSourceFactory::Etcd => {
                init_etcd_global(args.etcd_endpoints, args.etcd_username, args.etcd_password)
                    .await?;
                let ds = Arc::new(EtcdDataSource::new(mcp_cache));
                ds.clone().fetch_and_watch().await?;
                Ok(Arc::new(DataSourceEnum::Etcd(ds)))
            }
            DataSourceFactory::Mysql => {
                init_mysql_pool(&args.mysql_url).await?;
                let ds = Arc::new(MysqlDataSource::new(mcp_cache));
                ds.clone().fetch_and_watch().await?;
                Ok(Arc::new(DataSourceEnum::Redis(ds)))
            }
        }
    }
}
