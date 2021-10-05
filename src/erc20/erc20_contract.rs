use std::collections::HashMap;
use crate::common::*;


pub struct ERC20Contract{
    pub balances: HashMap<Address, Amount>,
    pub allowances: HashMap<Address, HashMap<Address, Amount>>,
    pub name: Name,
    pub symbol: Symbol,
    pub decimals: Decimals,
    pub total_supply: TotalSupply,

     
}
impl ERC20Contract{

    ///
    ///@dev Sets the values for {name} and {symbol}.
    ///
    ///The default value of {decimals} is 18. To select a different value for
    ///{decimals} you should overload it.
    ///
    ///All two of these values are immutable: they can only be set once during
    ///construction.
    ///
    pub fn constructor(name: Name, symbol: Symbol) -> Self{

        ERC20Contract{
            balances: HashMap::new(),
            allowances: HashMap::new(),
            name: name,
            symbol: symbol,
            decimals:0,
            total_supply: 0
        }
    }
    ///
    ///Returns the name of the token.
    ///
    pub fn name(&self) -> Name{
        return self.name.clone();
    }


    ///
    /// Returns the symbol of the token, usually a shorter version of the
    /// name.
    ///
    pub fn symbol(&self) -> Symbol {
        return self.symbol.clone();
    }


    ///
    ///@dev Returns the number of decimals used to get its user representation.
    ///For example, if `decimals` equals `2`, a balance of `505` tokens should
    ///be displayed to a user as `5.05` (`505 / 10 ** 2`).
    ///
    ///Tokens usually opt for a value of 18, imitating the relationship between
    ///Ether and Wei. This is the value {ERC20} uses, unless this function is
    ///overridden;
    ///NOTE: This information is only used for _display_ purposes: it in
    ///no way affects any of the arithmetic of the contract, including
    ///{IERC20-balanceOf} and {IERC20-transfer}.
    ///
    pub fn decimals(&self) -> Decimals {
        return self.decimals;
    }


   ///Returns the caller address (personwho calls ther contract)
    pub fn msg_sender(&self)-> Result<Address, ERC20Error>{
        let msg_err = ERC20Error::GetAddress(format!("addresses in msg_sender  MSG_SENDER doesn't exist {:?}", MSG_SENDER));
        Ok(ADDRESSES.get(MSG_SENDER).ok_or(msg_err)?.clone())
    }


    ///Returns the contract address
    pub fn this(&self)-> Result<Address, ERC20Error>{
        let msg_err = ERC20Error::GetAddress(format!("addresses in this() THIS doesn't exist {:?}", THIS));
        Ok(ADDRESSES.get(THIS).ok_or(msg_err)?.clone())
    }


    ///
    ///Moves `amount` of tokens from `sender` to `recipient`.
    ///
    ///This internal function is equivalent to {transfer}, and can be used to
    ///e.g. implement automatic token fees, slashing mechanisms, etc.
    ///
    ///Emits a {Transfer} event.
    ///
    ///Requirements:
    ///
    ///- `sender` cannot be the zero address.
    ///- `recipient` cannot be the zero address.
    ///- `sender` must have a balance of at least `amount`.
    ///
    pub(super) fn _transfer(&mut self, sender: &Address, recepient: &Address, amount: Amount) -> Result<(), ERC20Error>{

        if sender == "0"{
            return Err(ERC20Error::Transfer("ERC20: transfer from the zero address".to_string()));
        }
        if recepient == "0"{
            return Err(ERC20Error::Transfer("ERC20: transfer to the zero address".to_string()));
        } 
        let msg_err = ERC20Error::Transfer("balance_of no address".to_string());
    
        let sender_balance = self.balances.get_mut(sender).ok_or(msg_err)?;
        if *sender_balance < amount{
            return Err(ERC20Error::Transfer("ERC20: transfer amount exceeds balance".to_string()));
        }
        
        *sender_balance -= amount;
        let msg_err = ERC20Error::Transfer(format!("balances in _transfer  recepient doesn't exist {:?}", recepient));

        let balance_recepient = self.balances.get_mut(recepient).ok_or(msg_err)?;

        *balance_recepient += amount;
           
        // self.transfer_event(&sender, &recepient, amount).await;
        Ok(())
    }


    ///Creates `amount` tokens and assigns them to `account`, increasing
    ///the total supply.
    ///
    ///Emits a {Transfer} event with `from` set to the zero address.
    ///
    ///Requirements:
    ///
    ///- `account` cannot be the zero address.
    ///
    pub(super) fn _approve(&mut self, owner: &Address, spender: &Address, amount: Amount)-> Result<(), ERC20Error> {

        if owner == "0"{
            return Err(ERC20Error::Approve("ERC20: approve from the zero address".to_string()));
        }
        if spender == "0"{
            return Err(ERC20Error::Approve("ERC20: approve to the zero address".to_string()));
        }

        let spender_map = self.allowances.entry(owner.clone()).or_insert_with(|| {
            let mut x: HashMap<Address, Amount> = HashMap::new();
            x.insert(spender.clone(), amount);
            x
        });
        spender_map.entry(spender.clone()).or_insert(amount);

        Ok(())
    }

}





