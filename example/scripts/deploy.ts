import { ethers } from "hardhat";

async function main() {
  const Verify = await ethers.getContractFactory("Verify");
  const verify = await Verify.deploy();

  await verify.deployed();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
