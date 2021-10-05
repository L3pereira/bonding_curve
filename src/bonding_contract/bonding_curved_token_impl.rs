
// use async_trait::async_trait;
use super::bonding_curved_token::BondingCurvedToken;
use crate::common::*;
use crate::erc20::ierc20::IERC20;


// #[async_trait]
impl IERC20 for BondingCurvedToken  {

    fn total_supply(&mut self) -> Result<&mut TotalSupply, ERC20Error> {
        self.erc20_contract.total_supply()
    }

    fn balance_of(&mut self, address: &Address) -> Result<&mut Amount, ERC20Error> {
        self.erc20_contract.balance_of(address)
    }

    fn allowance(&self, owner: &Address, spender: &Address)  -> Result<Amount, ERC20Error>{
        self.erc20_contract.allowance(owner, spender)
    }

    fn approve(&mut self, spender: &Address, amount: Amount) -> Result<bool, ERC20Error>{
        self.erc20_contract.approve(spender, amount)
    }

    fn transfer(&mut self, address: &Address, amount: Amount) -> Result<bool, ERC20Error>{
        self.erc20_contract.transfer(address, amount)
    }

    fn transfer_from(&mut self, sender: &Address, recipient: &Address,  amount: Amount) -> Result<bool, ERC20Error>{
        self.erc20_contract.transfer_from(sender, recipient, amount)
    }
    // async fn transfer_event(&self, from: &Address, to: &Address, value: Amount){
    //     todo!();
    // }

    // async fn approval_event(&self, owner: &Address, spender: &Address, value: Amount){
    //     todo!();
    // }
}