import Web3 from "web3";
import { AbiItem } from "web3-utils";
import { Transaction as Tx } from "ethereumjs-tx";
import { walletAddress, privateKey, endpoint } from "../inputs/const";

const deploy = async () => {
  const web3Provider = new Web3.providers.HttpProvider(endpoint);
  const web3 = new Web3(web3Provider);
};
