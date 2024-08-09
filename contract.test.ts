import { test } from "vitest";
import { FSWorld } from "xsuite";

test("Test", async () => {
  using world = await FSWorld.start({
    // binaryPath: "path/to/binary"
  });
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
  }).assertFail({ code: "returnMessage", message: "not enough gas" });
});
