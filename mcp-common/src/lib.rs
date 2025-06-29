use clap::Parser;

pub mod cache;
pub mod constants;
pub mod etcd;
pub mod http_client;
pub mod provider;
pub mod xds;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct DynMcpArgs {
    #[arg(long, default_value = "0.0.0.0:9999")]
    pub addr: String,
    #[arg(long, value_delimiter = ',', default_value = "http://localhost:2379")]
    pub etcd_endpoints: Vec<String>,
    #[arg(long, default_value = "")]
    pub etcd_username: String,
    #[arg(long, default_value = "")]
    pub etcd_password: String,
    #[arg(long, default_value = "mysql")]
    pub data_source: String,
    #[arg(long, default_value = "")]
    pub mysql_url: String,
}
