use cosmwasm_std::{HumanAddr, Uint128, Decimal, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::snip721::{Trait};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
   pub white_members : Vec<HumanAddr>,
   pub admin : HumanAddr,
   pub total_supply:Uint128,
   pub maximum_count:Uint128,
   pub public_price:Uint128,
   pub private_price:Uint128,
   pub reward_wallet:Vec<Wallet>,
   pub token_address:HumanAddr,
   pub token_contract_hash:String,
   pub check_minted : Vec<bool>,
   pub human_metadata: String,
   pub human_image:String,
   pub robot_metadata:String,
   pub robot_image:String,
   pub bull_metadata:String,
   pub bull_image:String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    
    Receive{sender:HumanAddr,from:HumanAddr,amount:Uint128,msg:Binary},
    SetTotalSupply{amount: Uint128},
    SetMaximumNft{amount:Uint128},
    SetPrice{public_price:Uint128,private_price:Uint128},
    SetRewardWallet{wallet : Vec<Wallet>},
    ChangeAdmin{address:HumanAddr},
    SetSaleFlag{private_mint:bool,public_mint:bool},
    SetWhiteUsers{members:Vec<HumanAddr>},
    AddWhiteUser{member:HumanAddr},
    SetNftAddress{nft_address:HumanAddr,nft_contract_hash:String},
    SetTokenAddres{token_address:HumanAddr,token_contract_hash:String},
    AddMetaData {key:String,metadata: Vec<String>},
    SetMetaData {metadata: Vec<String>},
    SetRandom{},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetStateInfo {},
    GetWhiteUsers{},
    GetUserInfo{address:HumanAddr},
    GetMetadata{key:String},
    GetRand{}
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Wallet {
    pub address: HumanAddr,
    pub portion : Decimal
}


#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct MetadataMsg {
    pub tokenId:Option<String>,
     /// name of the item
    pub name: Option<String>,
    /// item description
    pub description: Option<String>,
    /// item attributes
    pub attributes: Option<Vec<Trait>>,
   
    /// url to the image
    pub image: Option<String>,  
  
}

