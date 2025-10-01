import { Hono } from "hono";
import { agentCall } from "@neardefi/shade-agent-js";
import { getQuote } from "../utils/fetch-quote";

const app = new Hono();

app.get("/", async (c) => {
  try {
    const data = await getQuote();

    if (!data.quote) {
      return c.json({ error: "Failed to get the quote" }, 500);
    }
    
    await agentCall({
      methodName: "trade",
      args: {
        receiver_id: data.quote.depositAddress,
        token_id: data.quoteRequest.originAsset,
        amount: data.quote.amountIn,
      },
    });

    return c.json({
      success: true,
    });
  } catch (error) {
    console.error("Failed to make the swap", error);
    return c.json({ error: "Failed to make the swap" }, 500);
  }
});

export default app;