use std::fmt::Debug;
use std::sync::Arc;

use async_trait::*;

use crate::prelude::EthereumAdapter;

pub mod blockchains;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetworkInstanceId {
  pub network: String,
  pub name: String,
}

#[async_trait]
pub trait NetworkInstance: Send + Sync {
  fn id(&self) -> &NetworkInstanceId;
  fn url(&self) -> &str;

  /// FIXME: The following methods are only necessary for
  /// backwards-compatibility. They will be removed as soon as we have abstracted
  /// away Ethereum in the rest of the codebase.

  fn compat_ethereum_adapter(&self) -> Option<Arc<dyn EthereumAdapter>>;
}

#[async_trait]
pub trait NetworkRegistry {
  fn register_instance(&mut self, chain: Box<dyn NetworkInstance>);
  fn instances(&self, network: &str) -> Vec<&Box<dyn NetworkInstance>>;
  fn instance(&self, id: NetworkInstanceId) -> Option<&Box<dyn NetworkInstance>>;
}
