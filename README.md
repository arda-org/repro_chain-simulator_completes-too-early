FIXED ✅

Scenario that fails:
- User calls a contract that tries to issue a token but with an incorrect fee
- The chain simulator says the transaction is processed but the callback is not executed yet

## How to reproduce

```
npm install

npm run build

npm run test
```

## The contract

The contract ([contract.rs](./contract.rs)):

```
pub trait Contract {
    ...

    #[only_owner]
    #[endpoint(issue)]
    #[payable("EGLD")]
    fn issue(&self) {
        let payment = self.call_value().egld_value().clone_value();
        self.send().esdt_system_sc_proxy()
            .issue_fungible(
                payment.clone(),
                &ManagedBuffer::new_from_bytes("TEST".as_bytes()),
                &ManagedBuffer::new_from_bytes("TEST".as_bytes()),
                &BigUint::zero(),
                FungibleTokenProperties::default(),
            )
            .async_call_and_exit();
    }

    #[storage_mapper("token")]
    fn token(&self) -> FungibleTokenMapper<Self::Api>;
}
```

## The test

The test ([./contract.test.rs](./contract.test.ts)):

```
test("Test", async () => {
  using world = await FSWorld.start();
  const deployer = await world.createWallet({
    balance: 10n ** 18n,
  });
  const { contract } = await deployer.deployContract({
    code: "file:output/contract.wasm",
    codeMetadata: [],
    gasLimit: 100_000_000,
  });
  await deployer.callContract({
    callee: contract,
    funcName: "issue",
    value: 10n ** 16n,
    gasLimit: 10_000_000,
  });
  assertAccount(await contract.getAccount(), {
    kvs: [],
  });
});
```
