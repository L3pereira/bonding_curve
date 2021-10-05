use std::collections::HashMap;
use lazy_static::lazy_static;
use thiserror::Error;
pub(crate) const THIS: &str = "this";
pub(crate) const MSG_SENDER: &str = "msg.sender";


lazy_static! {
    pub(crate) static ref ADDRESSES: HashMap<String, Address> = {
        let mut m = HashMap::new();
        m.insert(MSG_SENDER.to_string(), "My Address".to_string());
        m.insert(THIS.to_string(), "Contract Address".to_string());
        m
    };
}

pub type Address = String;
pub type Name = String;
pub type Symbol = String;
pub type Amount = u128;
pub type TotalSupply = u128;
pub type Decimals = u8;


#[derive(Error, Debug, PartialEq, Eq)]
pub enum BondingContractError {

    #[error("{0}")]
    PriceToMint(String),

    #[error("{0}")]
    PriceToBurn(String),

    #[error("{0}")]
    Burn(String),

    #[error("{0}")]
    Mint(String),

    #[error("{0}")]
    ERC20(#[from] ERC20Error),
    
}


#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ERC20Error {

    #[error("{0}")]
    Allowence(String),

    #[error("{0}")]
    Approve(String),

    #[error("{0}")]
    GetAddress(String),

    #[error("{0}")]
    Transfer(String),

    #[error("{0}")]
    TransferFrom(String),

}

