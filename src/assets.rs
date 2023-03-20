use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, StdError, StdResult, Uint128};
use std::collections::{HashMap, HashSet};

#[cw_serde]
pub struct VaultInfo {
    pub contract_addr: Addr,
    pub asset_infos: Vec<AssetInfo>,
    pub stratgy_infos: Vec<Addr>,
}

#[cw_serde]
#[derive(Hash, Eq)]
pub enum AssetInfo {
    /// Non-native Token
    Token { contract_addr: Addr },
    /// Native token
    NativeToken { denom: String },
}

#[cw_serde]
pub struct Asset {
    pub info: AssetInfo,
    pub amount: Uint128,
}

pub trait CoinsExt {
    fn assert_coins_properly_sent(
        &self,
        assets: &[Asset],
        vault_asset_infos: &[AssetInfo],
    ) -> StdResult<()>;
    fn assert_coin_ratio(&self) -> StdResult<()>;

    fn calc_shares(&self) -> Uint128;
}

impl CoinsExt for Vec<Coin> {
    fn assert_coins_properly_sent(
        &self,
        input_assets: &[Asset],
        vault_asset_infos: &[AssetInfo],
    ) -> StdResult<()> {
        let pool_coins = vault_asset_infos
            .iter()
            .filter_map(|asset_info| match asset_info {
                AssetInfo::NativeToken { denom } => Some(denom.to_string()),
                _ => None,
            })
            .collect::<HashSet<_>>();

        let input_coins = input_assets
            .iter()
            .filter_map(|asset| match &asset.info {
                AssetInfo::NativeToken { denom } => Some((denom.to_string(), asset.amount)),
                _ => None,
            })
            .map(|pair| {
                if pool_coins.contains(&pair.0) {
                    Ok(pair)
                } else {
                    Err(StdError::generic_err(format!(
                        "Asset {} is not supported",
                        pair.0
                    )))
                }
            })
            .collect::<StdResult<HashMap<_, _>>>()?;

        self.iter().try_for_each(|coin| {
            if input_coins.contains_key(&coin.denom) {
                if input_coins[&coin.denom] == coin.amount {
                    Ok(())
                } else {
                    Err(StdError::generic_err(
                        "Native token balance mismatch between the argument and the transferred",
                    ))
                }
            } else {
                Err(StdError::generic_err(format!(
                    "Supplied coins contain {} that is not in the input asset vector",
                    coin.denom
                )))
            }
        })
    }

    fn assert_coin_ratio(&self) -> StdResult<()> {
        match (&self[0].amount, &self[1].amount) {
            (a, b) if a == b => Ok(()),
            _ => Err(StdError::generic_err("Only Ratio 1:1 is supported")),
        }
    }
    fn calc_shares(&self) -> Uint128 {
        self[0].amount.checked_add(self[1].amount).unwrap()
    }
}
