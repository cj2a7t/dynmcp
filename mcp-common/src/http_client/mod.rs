pub mod http_client_provider;
pub mod model;

use once_cell::sync::Lazy;

use crate::http_client::http_client_provider::HttpClientProvider;


pub static HTTP_CLIENT: Lazy<HttpClientProvider> = Lazy::new(|| {
    HttpClientProvider::new().expect("Failed to initialize global HttpClientProvider")
});
