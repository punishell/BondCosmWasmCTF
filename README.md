# bond_cw_ctf

The bond contract allows users to deposit 2 tokens in a 1:1 ratio, and deposits those tokens into two separate strategy contracts, one per deposit denom.

---

## InstantiateMsg

Initializes the contract with required assets and contracts used for strategies.

```json
{
  "owner": "wasm...",
  "asset_infos": "[native_token:{denom:ATOM}, native_token:{denom:OSMO},]",
  "strategy_infos": ["wasm...", "wasm..."],

}
```

## ExecuteMsg

### `deposit`

Deposit allow user to deposit the assets in ration 1:1.

```json
{
  "deposit": {
    "assets":["ATOM","OSMO"]
  }
}
```

### `Bond`

Bond not yet bonded assets to strategies in 1:1 ratio.

```json
{
  "bond": {}
}
```

### `StartUnbond`

StartUnbond allow user to start ubond proces of the bonded assets.

```json
{
  "start_unbond": {
    "id": "...",
    "amount": 123
  }
}
```

### `Unbond`

Unbond allow user to unbond assets provided in StartUnbond after lock period  is reached.

```json
{
  "unbond": {
      "id": "..."
  }
}
```
