// use async_trait::async_trait;
use super::erc20_contract::ERC20Contract;
use super::ierc20::IERC20;
use crate::common::*;

// #[async_trait]
impl IERC20 for ERC20Contract {

    fn total_supply(&mut self) -> Result<&mut TotalSupply, ERC20Error>{
        Ok(&mut self.total_supply)
    }

    fn balance_of(&mut self, address: &Address) -> Result<&mut Amount, ERC20Error>{
        Ok(self.balances.entry(address.clone()).or_insert(0))
    }

    fn allowance(&self, owner: &Address, spender: &Address)  -> Result<Amount, ERC20Error>{
        let msg_err_1 = ERC20Error::Allowence(format!("allowances in allowance  owner doesn't exist {:?}", owner));
        let msg_err_2 = ERC20Error::Allowence(format!("allowances in allowance  spender doesn't exist {:?}", spender));
        Ok(self.allowances.get(owner).ok_or(msg_err_1)?.get(spender).ok_or(msg_err_2)?.clone())
    }

    fn approve(&mut self, spender: &Address, amount: Amount) -> Result<bool, ERC20Error>{

        self._approve(&self.msg_sender()?, spender, amount)?;
        Ok(true)
    }

    fn transfer(&mut self, recipient: &Address, amount: Amount) -> Result<bool, ERC20Error>{
        self._transfer(&self.msg_sender()?, recipient, amount)?;
        Ok(true)
    }

    fn transfer_from(&mut self, sender: &Address, recipient: &Address,  amount: Amount) -> Result<bool, ERC20Error>{
        self._transfer(sender, recipient, amount)?;
        let msg_sender = &self.msg_sender()?;
        let current_allowance: u128 = self.allowance(sender, msg_sender)?;
        if current_allowance < amount {
            return Err(ERC20Error::TransferFrom("ERC20: transfer amount exceeds allowance".to_string()));
        }
        self._approve(sender, msg_sender, current_allowance - amount)?;
        Ok(true)
    }
    
    // async fn transfer_event(&self, from: &Address, to: &Address, value: Amount){
    //     todo!();
    // }

    // async fn approval_event(&self, owner: &Address, spender: &Address, value: Amount){
    //     todo!();
    // }
}