#![deny(
    //  missing_docs, // not compatible with big_array
      trivial_casts,
      trivial_numeric_casts,
      unsafe_code,
      unused_import_braces,
      unused_qualifications,
      warnings
  )]

pub mod erc20;
pub mod bonding_contract;
pub mod common;

#[cfg(test)]
mod tests {
    use super::bonding_contract::bonding_curved_token::*;
    use crate::erc20::ierc20::IERC20;
    use super::common::{ADDRESSES, THIS, MSG_SENDER};
    use pretty_assertions::assert_eq;


    fn init(sender_add: &str) -> BondingCurvedToken{

        let mut token = BondingCurvedToken::constructor("My token".to_string(), "MT".to_string(), 0);

        let _ = token.approve(&sender_add.to_owned(), 10000);
        token
    }

    #[test]
    fn test_price_to_mint() {
        let sender_add = ADDRESSES.get(MSG_SENDER).unwrap();

        let mut token = init(sender_add);
        let result = token.price_to_mint(10);
        assert_eq!(result, Ok(520));
        let result = token.price_to_mint(20);
        assert_eq!(result, Ok(2040));

    }
    #[test]
    fn test_zero_balance_reward_for_burn() {

        let sender_add = ADDRESSES.get(MSG_SENDER).unwrap();
        let mut token = init(sender_add);    
        let result = token.reward_for_burn(1);

        assert_eq!(result.is_err(), true);
    }


    #[test]
    fn test_allowance() {
        let sender_add = ADDRESSES.get(MSG_SENDER).unwrap();
        let owner_add = ADDRESSES.get(THIS).unwrap();
        let mut token = init(sender_add); 
        let _ = token.approve(&owner_add.to_owned(), 10000);
        let result = token.allowance(&sender_add.to_owned(), &owner_add.to_owned());
        assert_eq!(result, Ok(10000));
    }

    #[test]
    fn test_approve() {
        let sender_add = ADDRESSES.get(MSG_SENDER).unwrap();
 
        let mut token = init(sender_add); 
        let result = token.approve(&sender_add.to_owned(), 20000);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_mint() {
        let sender_add = ADDRESSES.get(MSG_SENDER).unwrap();
  
        let mut token = init(sender_add); 

        let _ = token.mint(10);

        assert_eq!(token.pool_balance, 520);

        let _ = token.mint(10);

        assert_eq!(token.pool_balance, 2040);
    }

    #[test]
    fn test_burn() {
        let sender_add = ADDRESSES.get(MSG_SENDER).unwrap();

        let mut token = init(sender_add); 

        let _ = token.mint(20);

        assert_eq!(token.pool_balance, 2040);

        let _ = token.burn(10);

        assert_eq!(token.pool_balance, 520);
    }

    #[test]
    fn test_negative_burn() {
        let sender_add = ADDRESSES.get(MSG_SENDER).unwrap();

        let mut token = init(sender_add); 

        assert_eq!(token.burn(2000).is_err(), true);
    }

    #[test]
    fn test_reward_for_burn() {
        let sender_add = ADDRESSES.get(MSG_SENDER).unwrap();
        let mut token = init(sender_add);
        let _ = token.mint(20); 
        token.pool_balance = 2040;

        assert_eq!(token.reward_for_burn(10), Ok(1520));
    }

    #[test]
    fn test_balance_of() {
        let sender_add = ADDRESSES.get(MSG_SENDER).unwrap();
        let mut token = init(sender_add); 
        let _ = token.mint(10000);
        let result = token.balance_of(&sender_add.to_owned());
        assert_eq!(result, Ok(&mut 10000));
    }
}
