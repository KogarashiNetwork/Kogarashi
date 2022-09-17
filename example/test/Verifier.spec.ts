import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";
import { confidentialTransferInputs } from "../inputs/const";

describe("Verifier", function () {
  async function deployOneYearLockFixture() {
    const [owner, otherAccount] = await ethers.getSigners();

    const Verifier = await ethers.getContractFactory("Verifier");
    const verifier = await Verifier.deploy();

    return { verifier, owner, otherAccount };
  }

  describe("Deployment", function () {
    it("Should verify proof", async function () {
      const { a, b, c, input } = confidentialTransferInputs;
      const { verifier } = await loadFixture(deployOneYearLockFixture);

      expect(await verifier.verifyProof(a, b, c, input)).to.equal(true);
    });
  });
});
