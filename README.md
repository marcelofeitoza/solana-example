# Running

## Using already deployed contract

If already with the target folder, try running:

```bash
yarn update-idl && yarn update-types
```

Then, you can run the project with:

```bash
cd sdk/ && yarn start
```

## Deploying the contract

If you want to deploy the contract, you can run:

```bash
cargo clean # Clean the target folder
anchor build 
anchor deploy
```

The last command `anchor deploy` will output the Program ID, which MUST be updated in the `programs/devolt/src/lib.rs` file, `Anchor.toml` file and in the `sdk/src/constants/program.ts` files.

After that, you can run the commands above to run the project (updating the IDL and types then running the project).