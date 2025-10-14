// Fetch quote from 1click.chaindefuser.com with fixed values
export async function getQuote() {
  const url = 'https://1click.chaindefuser.com/v0/quote';
  
  const contractId = process.env.NEXT_PUBLIC_contractId;
  
  const requestBody = {
    dry: false,
    depositMode: "SIMPLE",
    swapType: "EXACT_INPUT",
    slippageTolerance: 100,
    originAsset: "nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near", // ETH
    depositType: "INTENTS",
    destinationAsset: "nep141:wrap.near", // USDC on NEAR 
    amount: "100000",
    refundTo: contractId,
    refundType: "INTENTS",
    recipient: contractId,
    recipientType: "INTENTS",
    deadline: "2025-10-02T14:15:22Z",
    referral: "referral"
  };

  try {
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': '*/*'
      },
      body: JSON.stringify(requestBody)
    });

    const data = await response.json();
    
    if (!response.ok) {
      console.error('Quote API Error:', data);
      return null;
    }
    
    console.log("data", data);
    return data;
  } catch (error) {
    console.error('Network error fetching quote:', error);
    return null;
  }
}


