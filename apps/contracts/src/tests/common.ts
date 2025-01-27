import { Address } from "@stellar/stellar-sdk";
import { SOROSWAP_USDC, SOROSWAP_XTAR } from "../constants.js";


export const usdcAddress = new Address(SOROSWAP_USDC);
export const xtarAddress = new Address(SOROSWAP_XTAR);

export const yellow = "\x1b[33m%s\x1b[0m";
export const green = "\x1b[32m%s\x1b[0m";
export const purple = "\x1b[35m%s\x1b[0m";
export const red = "\x1b[31m%s\x1b[0m";