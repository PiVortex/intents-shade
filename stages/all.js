// USDC in intents
// nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1
// USDC contract
// 17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1
// USDC decimals 
// 6 

// BTC in intents 
// nep141:btc.omft.near
// BTC decimals 
// 8 



import { Account } from "@near-js/accounts";
import { KeyPairSigner } from "@near-js/signers";
import { parseSeedPhrase } from "near-seed-phrase";
import { JsonRpcProvider } from "@near-js/providers";

const seedPhrase = "salad dad garment boy enforce promote void reunion arrange company mutual over";
const accountId = "ac-proxy.shade-intents.near"
const url = 'https://1click.chaindefuser.com/v0/quote';

// 
const provider = new JsonRpcProvider({ url: "https://free.rpc.fastnear.com"});
const { secretKey } = parseSeedPhrase(seedPhrase);
const signer = KeyPairSigner.fromSecretKey(secretKey);
const account = new Account(accountId, provider, signer);

// Deposit USDC to intents 
async function deposit() {
  await account.callFunction({
    contractId: "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1", // USDC 
    methodName: "ft_transfer_call", 
    args: {
      receiver_id: "intents.near",
      amount: "100000", // 0.1 USDC
      msg: ""
    },
    deposit: "1",
    gas: "100000000000000",
  });
}

async function check_intents_balance() {
  const usdc_amount = await provider.callFunction("intents.near","mt_balance_of", {
      account_id: "ac-proxy.shade-intents.near",
      token_id: "nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1"
    }
  )
  console.log("USDC plain amount : ", usdc_amount)
  console.log("USDC formatted amount: ", usdc_amount / 10**6)
  
  const btc_amount = await provider.callFunction("intents.near","mt_balance_of", {
      account_id: "ac-proxy.shade-intents.near",
      token_id: "nep141:btc.omft.near"
    }
  )
  console.log("BTC plain amount : ", btc_amount)
  console.log("BTC formatted amount: ", btc_amount / 10**8)
  }

// Swap USDC for BTC
async function swap() {
  const requestBody = {
    dry: false,
    depositMode: "SIMPLE",
    swapType: "EXACT_INPUT",
    slippageTolerance: 100,
    originAsset: "nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1", // USDC
    depositType: "INTENTS",
    destinationAsset: "nep141:btc.omft.near", // BTC 
    amount: "50000",
    refundTo: accountId,
    refundType: "INTENTS",
    recipient: accountId,
    recipientType: "INTENTS",
    deadline: "2025-10-05T14:15:22Z",
    referral: "referral"
  };
  
  const response = await fetch(url, {
  method: 'POST',
  headers: {
      'Content-Type': 'application/json',
      'Accept': '*/*'
  },
  body: JSON.stringify(requestBody)
  });
  
  const data = await response.json();
  
  await account.callFunction({
    contractId: "intents.near",
    methodName: "mt_transfer", 
    args: {
      receiver_id: data.quote.depositAddress, // Deposit address specified by the 1Click API quote
      token_id: data.quoteRequest.originAsset, // Asset swapping from
      amount: data.quote.amountIn, // Amount of asset I'm swapping
    },
    deposit: "1"
  });
  
}

// Withdraw USDC from intents 
async function withdraw() {
  await account.callFunction({
    contractId: "intents.near",
    methodName: "ft_withdraw", 
    args: {
      token: "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
      receiver_id: accountId,
      amount: "50000",
    },
    deposit: "1",
  });
}

// deposit()
// check_intents_balance()
// swap()
// withdraw()
