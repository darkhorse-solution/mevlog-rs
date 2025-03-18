use std::fmt;

use eyre::Result;
use revm::primitives::Address;

use crate::misc::{
    symbol_utils::{symbol_lookup_cached_async, SymbolLookupWorker},
    utils::UNKNOWN,
};

#[derive(Debug, Clone)]
pub struct MEVLogSignature {
    pub signature: String,
    symbol: Option<String>,
}

#[derive(Debug)]
pub enum MEVLogSignatureType {
    ERC20,
    UNIV2,
    UNIV3,
}

impl MEVLogSignature {
    pub async fn new(
        address: Address,
        signature_str: Option<String>,
        symbols_lookup: &SymbolLookupWorker,
    ) -> Result<Self> {
        let signature_str = signature_str.unwrap_or(UNKNOWN.to_string());
        let signature_type = get_signature_type(&signature_str);

        let symbol = symbol_lookup_cached_async(address, signature_type, symbols_lookup).await?;

        Ok(Self {
            signature: signature_str,
            symbol,
        })
    }
}

fn get_signature_type(signature_str: &str) -> Option<MEVLogSignatureType> {
    match signature_str {
        "Transfer(address,address,uint256)" => Some(MEVLogSignatureType::ERC20),
        "Approval(address,address,uint256)" => Some(MEVLogSignatureType::ERC20),
        "Swap(address,uint256,uint256,uint256,uint256,address)" => Some(MEVLogSignatureType::UNIV2),
        "Swap(address,address,int256,int256,uint160,uint128,int24)" => {
            Some(MEVLogSignatureType::UNIV3)
        }
        _ => None,
    }
}

impl fmt::Display for MEVLogSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.signature,
            self.symbol.as_deref().unwrap_or_default()
        )
    }
}
