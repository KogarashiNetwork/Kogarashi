import { BigNumber, BigNumberish } from "ethers";

interface ConfidentialTransferInputs {
  a: [BigNumberish, BigNumberish];
  b: [[BigNumberish, BigNumberish], [BigNumberish, BigNumberish]];
  c: [BigNumberish, BigNumberish];
  input: BigNumberish[];
}

export const confidentialTransferInputs: ConfidentialTransferInputs = {
  a: [
    BigNumber.from(
      "0x1ab1bf78613c5b4fcd2fda943c6f88be4b7e9b518397bfa42932c97dfc2cd50b"
    ),
    BigNumber.from(
      "0x29070457351973ca03ff841e88986f4489fcaba637c941307cad3264feaf97d8"
    ),
  ],
  b: [
    [
      BigNumber.from(
        "0x0ed1c343a503dd416467cc871eef966f9810e28444f409f9cc43ffb380c3804d"
      ),
      BigNumber.from(
        "0x2675f52f005c851a6d0426c6a607f4d51a74b48de0b58177af9e34aeaf7134df"
      ),
    ],
    [
      BigNumber.from(
        "0x139f6aa940502c75d582d4d2c10b9d752b96cd4f18f107ab4f77ec6edaeae946"
      ),
      BigNumber.from(
        "0x26f53de8211ae3ebce309e2b7517516b518b7b44fea35c4f8d5ad9a00d4a1c06"
      ),
    ],
  ],
  c: [
    BigNumber.from(
      "0x12a1afb3f28038be6a5111265696b295c3e7fa0c9a8a785657764c52a4d80f4b"
    ),
    BigNumber.from(
      "0x2e6209d22a41ceabf6016b8a38553ceb7616be4c6518ba8e25ac49162a07e79d"
    ),
  ],
  input: [
    BigNumber.from(
      "0x2ec37d71fbc59c1027eeec7493565a89e0507746f816c6e9d9b3881f5896ee13"
    ),
    BigNumber.from(
      "0x0f09585b0ef177330a839815bd0855a0a524f6c3d3d3ba4b2a7243269c8a32db"
    ),
    BigNumber.from(
      "0x175cb4b70ed75b12ca1e824497c2386218c2261005c45c477b7c54549c6a9e5a"
    ),
    BigNumber.from(
      "0x11cdbc2d7d1c35fe5e55b74d54664c4f57fbc07ff3bcb09ad7470078e6ca85cd"
    ),
    BigNumber.from(
      "0x1e4b9ec20ccdca6b5fba3ea1108f78a290fff019aa7af78f9aa5049853dfeb79"
    ),
    BigNumber.from(
      "0x0d0ab1b2b79e55d7f5e58cf56a18f47cf9dc892ad57a4a4bc1a2205c9252d3ad"
    ),
    BigNumber.from(
      "0x2b441370ce366b50a02cfdbeafd27dac9232b56460988215ef004d176f35c691"
    ),
    BigNumber.from(
      "0x0d0ab1b2b79e55d7f5e58cf56a18f47cf9dc892ad57a4a4bc1a2205c9252d3ad"
    ),
    BigNumber.from(
      "0x0000000000000000000000000000000000000000000000000000000000000005"
    ),
  ],
};

export const walletAddress = "0xa9d873C915a2DDdA0Be05D0bfa83c245b4B1002C";

export const privateKey =
  "6f35a186477bc4045d9cbbe077a18a19af41eb0a11cf751b716443b184f974b5";

export const endpoint = process.env.ENDPOINT || "http://localhost:8545";
