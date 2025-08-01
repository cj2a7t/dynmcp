use anyhow::{Error, Result};
use async_trait::async_trait;
use std::sync::Arc;

use crate::datasource::{
    datasource::DataSource, etcd_ds::EtcdDataSource, mysql_ds::MysqlDataSource,
};

pub enum DataSourceEnum {
    Etcd(Arc<EtcdDataSource>),
    Mysql(Arc<MysqlDataSource>),
}

#[async_trait]
impl DataSource for DataSourceEnum {
    async fn fetch_and_watch(self: Arc<Self>) -> Result<()> {
        match self.as_ref() {
            DataSourceEnum::Etcd(ds) => ds.clone().fetch_and_watch().await,
            DataSourceEnum::Mysql(ds) => ds.clone().fetch_and_watch().await,
        }
    }

    async fn put<T: serde::Serialize + Clone + Send + Sync + 'static>(
        self: Arc<Self>,
        id: &str,
        value: &T,
    ) -> Result<T, Error> {
        match self.as_ref() {
            DataSourceEnum::Etcd(ds) => ds.clone().put(id, value).await,
            DataSourceEnum::Mysql(ds) => ds.clone().put(id, value).await,
        }
    }

    async fn get<T: for<'de> serde::Deserialize<'de>>(
        self: Arc<Self>,
        id: &str,
    ) -> Result<T, Error> {
        match self.as_ref() {
            DataSourceEnum::Etcd(ds) => ds.clone().get(id).await,
            DataSourceEnum::Mysql(ds) => ds.clone().get(id).await,
        }
    }

    async fn delete(self: Arc<Self>, id: &str) -> Result<bool, Error> {
        match self.as_ref() {
            DataSourceEnum::Etcd(ds) => ds.clone().delete(id).await,
            DataSourceEnum::Mysql(ds) => ds.clone().delete(id).await,
        }
    }

    async fn get_all<T>(self: Arc<Self>) -> Result<Vec<T>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        match self.as_ref() {
            DataSourceEnum::Etcd(ds) => ds.clone().get_all().await,
            DataSourceEnum::Mysql(ds) => ds.clone().get_all().await,
        }
    }
}
