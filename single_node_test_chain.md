
# 🚀 How to Make a Single-Node Chain as a Test Chain

## 1. Build the Initial Chain Spec
Run the following command to generate the initial chain specification:

```bash
./target/release/fintra-node build-spec --disable-default-bootnode > fintra-spec.json
```

## 2. Edit the Chain Spec
Open the `fintra-spec.json` file and **remove** the `aura` and `grandpa` sections under `"genesis.runtime"`.  
We will use session keys instead, which include both Aura and Grandpa.

## 3. Get Aura and Grandpa Keys
Use `subkey` to inspect the required keys for Alice:

```bash
subkey inspect //Alice --scheme sr25519    # For Aura
subkey inspect //Alice --scheme ed25519    # For Grandpa
```

## 4. Add Session Keys to the Spec
In the `fintra-spec.json`, insert the session keys section like below under `"genesis.runtime"`:

```json
"session": {
  "keys": [
    [
      "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      {
        "aura": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "grandpa": "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
      }
    ]
  ]
}
```

## 5. Generate Raw Chain Spec
Run the following command to create a raw spec file:

```bash
./target/release/fintra-node build-spec --chain fintra-spec.json --raw > fintra-spec-raw.json
```

## 6. Create a Folder for Blockchain Data
Create a directory to store chain data (replace path as needed):

```bash
mkdir -p /Users/air/Documents/fintra-bloc1
```

## 7. Start the Node
Now start the node with:

```bash
./target/release/fintra-node \
  --base-path /Users/air/Documents/fintra-bloc1 \
  --chain fintra-spec-raw.json \
  --validator \
  --alice \
  --rpc-external \
  --rpc-cors=all \
  --rpc-methods=Unsafe
```

## 8. Fix "missing `secret_ed25519`" Error (If Any)
If you encounter an error related to missing `secret_ed25519`, follow these steps:

### a. Create the directory:
```bash
mkdir -p /Users/air/Documents/fintra-bloc1/chains/local_testnet/network/
```

### b. Generate an ed25519 key:
```bash
subkey generate --scheme ed25519
```

Copy the **public key** from the output (without the `0x` prefix).

### c. Store the key:
```bash
echo "60b399f637b342f27e52fd6295be803c08b9fc9fe72a907df60b6009698088fa" | xxd -r -p > /Users/air/Documents/fintra-bloc1/chains/local_testnet/network/secret_ed25519
```

*(Replace the hex string above with your actual public key, without the `0x` prefix.)*

## 9. Start the Node Again
After fixing the key issue, re-run the node:

```bash
./target/release/fintra-node \
  --base-path /Users/air/Documents/fintra-bloc1 \
  --chain fintra-spec-raw.json \
  --validator \
  --alice \
  --rpc-external \
  --rpc-cors=all \
  --rpc-methods=Unsafe
```
