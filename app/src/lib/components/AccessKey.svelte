<script lang="ts">
  import type { Account } from "@near-wallet-selector/core";
  import { FixedNumber } from "@tarnadas/fixed-number";
  import base58 from "bs58";

  import { browser } from "$app/environment";
  import { getAccessKey } from "$lib/auth";
  import { MessageBox } from "$lib/components";
  import { Section } from "$lib/layout";
  import type { AccessKeyReponse, Condition } from "$lib/models";

  export let account: Account;

  let accessKey: string | undefined;
  let publicKey: string | undefined;
  let allowance: FixedNumber | undefined;

  let condition: Condition;
  $: if (!publicKey || !allowance) {
    condition = "loading";
  } else if (allowance.toNumber() < 0.05) {
    condition = "warn";
  } else {
    condition = "ok";
  }

  if (browser) {
    getAccessKey().then((key) => {
      accessKey = key;
      if (accessKey) {
        registerKey(accessKey);
      }
    });
  } else {
    accessKey = undefined;
  }

  async function registerKey(accessKey: string) {
    const privateKey = base58.decode(accessKey.substring(8));
    publicKey = `ed25519:${base58.encode(privateKey.slice(32))}`;
    let retries = 0;
    while (retries < 10 && allowance == null) {
      try {
        const res = await fetch(import.meta.env.VITE_NODE_URL, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            jsonrpc: "2.0",
            id: "dontcare",
            method: "query",
            params: {
              request_type: "view_access_key",
              finality: "final",
              account_id: account.accountId,
              public_key: publicKey,
            },
          }),
        });
        const json = (await res.json()) as { result: AccessKeyReponse };
        if (!json.result) throw Error();
        allowance = new FixedNumber(
          json.result.permission.FunctionCall.allowance,
          24,
        );
      } catch (err) {
        await new Promise((r) => setTimeout(r, 2_000));
      } finally {
        retries++;
      }
    }
  }
</script>

<Section header="Access Key" {condition}>
  {#if publicKey && allowance}
    <div class="field">
      {publicKey.substring(0, 12)}...{publicKey.substr(-4)}
    </div>
    <div class="field">
      <span>Allowance:</span>
      <span>{allowance.format({ maximumFractionDigits: 4 })} NEAR</span>
    </div>
    {#if allowance.toNumber() < 0.05}
      <MessageBox type="warning">
        Allowance is critically low! Please logout and log back in to register a
        new access key.
      </MessageBox>
    {/if}
  {/if}
</Section>
