mod rpc;


use std::path::Path;

use serde::{
    self,
    Serialize,
    Deserialize,
};
use anyhow::{
    self,
    Context,
};

use rpc::RpcConfig;


fn default_view_client_threads() -> usize {
    4
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Config {
    pub archive: bool,
   
    #[serde(default = "default_view_client_threads")]
    pub view_client_threads: usize,

    pub rpc: Option<RpcConfig>,
  
}



impl Default for Config {
    fn default() -> Self {
        Config {
            rpc: Some(RpcConfig::default()),
            archive: false,
            view_client_threads:default_view_client_threads(),
        }
    }
}


impl Config {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let s = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config from {}", path.display()))?;
        let config =  toml::from_str(&s)
            .with_context(|| format!("Failed to parse toml config file from {}", path.display()))?;
        Ok(config)
    }

    pub fn rpc_addr(&self) -> Option<&str> {
        if let Some(rpc) = &self.rpc {
            return Some(&rpc.addr);
        }
        None
    }

    #[allow(unused_variables)]
    pub fn set_rpc_addr(&mut self, addr: String) {
        {
            self.rpc.get_or_insert(Default::default()).addr = addr;
        }
    }
}