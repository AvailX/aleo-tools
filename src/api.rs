pub mod blocking;
pub use blocking::*;

use snarkvm::prelude::*;

use std::marker::PhantomData;

/// Aleo API client for interacting with the Aleo Beacon API
#[derive(Clone, Debug)]
pub struct AleoAPIClient<N: Network> {
    client: ureq::Agent,
    base_url: String,
    network_id: String,
    _network: PhantomData<N>,
}

impl<N: Network> AleoAPIClient<N> {
    pub fn new(base_url: &str, chain: &str) -> Result<Self> {
        let client = ureq::Agent::new();
        ensure!(
            base_url.starts_with("http://") || base_url.starts_with("https://"),
            "specified url {base_url} invalid, the base url must start with or https:// (or http:// if doing local development)"
        );
        Ok(AleoAPIClient {
            client,
            base_url: base_url.to_string(),
            network_id: chain.to_string(),
            _network: PhantomData,
        })
    }

    pub fn testnet3() -> Self {
        Self::new("https://vm.aleo.org/api", "testnet3").unwrap()
    }

    pub fn local_testnet3(port: &str) -> Self {
        Self::new(&format!("http://localhost:{}", port), "testnet3").unwrap()
    }

    /// Get base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get network ID being interacted with
    pub fn network_id(&self) -> &str {
        &self.network_id
    }
}
