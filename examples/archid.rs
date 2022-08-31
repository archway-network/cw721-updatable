use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw721_updatable::Expiration;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Account {
  username: Option<String>,
  profile: Option<String>,
  account_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Website {
  url: Option<String>,
  domain: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Metadata {
    pub name: Option<String>,         // e.g. for interoperability with external marketplaces
    pub description: Option<String>,  // e.g. ibid.
    pub image: Option<String>,        // e.g. ibid.
    pub expiry: Option<Expiration>,
    pub domain: Option<String>,
    pub subdomains: Vec<String>,
    pub accounts: Vec<Account>,
    pub websites: Vec<Website>,
}

fn example_metadata() {

  let subdomains = vec![
    "game".to_string(), 
    "dapp".to_string(), 
    "market".to_string()
  ];

  let accounts = vec![
    Account {
      username: Some("drew.taylor@chainofinsight.com".to_string()),
      profile: None,
      account_type: Some("email".to_string()),
    },
    Account {
      username: Some("@chainofinsight".to_string()),
      profile: Some("twitter.com/chainofinsight".to_string()),
      account_type: Some("twitter".to_string()),
    }
  ];

  let websites = vec![
    Website {
      url: Some("drewstaylor.com".to_string()),
      domain: Some("drewstaylor.arch".to_string()),
    },
    Website {
      url: Some("game.drewstaylor.com".to_string()),
      domain: Some("game.drewstaylor.arch".to_string()),
    },
    Website {
      url: Some("dapp.drewstaylor.com".to_string()),
      domain: Some("dapp.drewstaylor.arch".to_string()),
    },
    Website {
      url: Some("market.drewstaylor.com".to_string()),
      domain: Some("market.drewstaylor.arch".to_string()),
    }
  ];

  let metadata_extension = Some(Metadata {
      name: Some("drewstaylor.arch".into()),
      description: Some("default token description".into()),
      image: Some("ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".into()),
      domain: Some("drewstaylor.arch".into()),
      expiry: Some(Expiration::AtHeight(1234567)),
      subdomains: subdomains,
      accounts: accounts,
      websites: websites,
  });

  dbg!(metadata_extension);
}

fn main() {
  example_metadata();
}