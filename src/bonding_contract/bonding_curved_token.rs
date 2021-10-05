
use crate::common::*;
use crate::erc20::erc20_contract::ERC20Contract;
use crate::erc20::ierc20::IERC20;


pub struct BondingCurvedToken{
    pub (super) erc20_contract: ERC20Contract,
    pub pool_balance: Amount
}
impl BondingCurvedToken{

    pub fn constructor(name: Name, symbol: Symbol, _: u8) -> Self {
        let contract = ERC20Contract::constructor(name.clone(), symbol.clone());
        
        BondingCurvedToken{
            erc20_contract: contract,
            pool_balance:0
        }
    }

    // async fn minted_event(&self, amount: Amount, total_cost: Amount){
    //     todo!();
    // }

    // async fn burned_event(&self, amount: Amount, total_cost: Amount){
    //     todo!();
    // }

    ///The exact quatity to mint or buy tokens is the area (integral calculation) bellow the functions
    fn integral_linear_curve(&self, x: u128) -> u128{
        //integral of y=10x+2 -> y=5x^2+2x
        5*x.pow(2)+2*x
    }


    ///The price (coins) to pay an amount of tokens 
    pub fn price_to_mint(&mut self, num_tokens: Amount) -> Result<Amount, BondingContractError>{
        let total_supply = *self.total_supply()?;
        let new_supply = total_supply + num_tokens;
        let integral_result =  self.integral_linear_curve(new_supply);

        if self.pool_balance > integral_result{
            let msg_err_1 = "ERC20: price_to_mint, integral_result cannot be lower than pool_balance".to_string();
            return Err(BondingContractError::PriceToMint(msg_err_1));
        }

        Ok(self.integral_linear_curve(new_supply) - self.pool_balance)
    }

    ///The price (coins) to to receive in exchange for an amount of tokens
    pub fn reward_for_burn(&mut self, num_tokens: Amount) -> Result<Amount, BondingContractError> {

        let total_supply = *self.total_supply()?;

        if num_tokens > total_supply{
            let msg_err = "ERC20: Reward for burn, num tokens cannot be higher than supply".to_string();
            return Err(BondingContractError::PriceToMint(msg_err));
        }
        let new_supply = total_supply - num_tokens;
        let ether_rewards = self.integral_linear_curve(new_supply);

        if ether_rewards > self.pool_balance{
            let msg_err = "ERC20: Reward for burn, Ether to reward cannot be higher than the pool balance".to_string();
            return Err(BondingContractError::PriceToMint(msg_err));
        }

        Ok(self.pool_balance - ether_rewards)
    }


    ///Mint tokens in exchange for coins
    pub fn mint(&mut self, num_tokens: Amount) -> Result<(), BondingContractError> {
        let price_for_tokens: Amount = self.price_to_mint(num_tokens)?;
   
        let msg_sender = &self.erc20_contract.msg_sender()?;
      
        let total_supply = self.total_supply()?;
 
        *total_supply += num_tokens;
 
        let balance  = self.balance_of(msg_sender)?;
  
        *balance += num_tokens;

        self.pool_balance += price_for_tokens;
        //Send exchange to msg.sender.transfer(msg.value - priceForTokens)!!!!!!!
        // self.minted_event(num_tokens, price_for_tokens).await;

        Ok(())
    }


    ///Burn tokens in exchange for coins
    pub fn burn(&mut self, num_tokens: Amount) -> Result<(), BondingContractError>{

        let ether_to_return = self.reward_for_burn(num_tokens)?;
        let msg_sender = &self.erc20_contract.msg_sender()?;
        let balance  = self.balance_of(msg_sender)?;

        if *balance < num_tokens{
            let msg_err = "ERC20: burn, Balance cannot be lower than num tokens".to_string();
            return Err(BondingContractError::Burn(msg_err));
        }
        *balance -= num_tokens;

        let total_supply = self.total_supply()?;
        *total_supply -= num_tokens;
  
        self.pool_balance -= ether_to_return;

        //Send ether_to_return to msg.sender.transfer(ether_to_return)!!!!!!!

        // self.burned_event(num_tokens, reserve_tokens_to_return).await
        Ok(())
    }
}