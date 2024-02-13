<script lang="ts">
  import type { Account } from "@near-wallet-selector/core";
  import { FixedNumber } from "@tarnadas/fixed-number";
  import base58 from "bs58";

  import { browser } from "$app/environment";
  import { account$, getAccessKey } from "$lib/auth";
  import { MessageBox } from "$lib/components";
  import ProgressSpinner from "$lib/components/ProgressSpinner.svelte";
  import type { AccessKeyReponse } from "$lib/models";

  let accessKey: string | undefined;
  let publicKey: string | undefined;
  let allowance: FixedNumber | undefined;

  $: if (browser && $account$) {
    accessKey = getAccessKey();

    if (accessKey) {
      registerKey($account$, accessKey);
    }
  } else {
    accessKey = undefined;
  }

  async function registerKey(account: Account, accessKey: string) {
    const privateKey = base58.decode(accessKey.substring(8));
    publicKey = `ed25519:${base58.encode(privateKey.slice(32))}`;
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
    if (!json.result) return;
    allowance = new FixedNumber(
      json.result.permission.FunctionCall.allowance,
      24,
    );
  }
</script>

<div class="page">
  <h2>Your Access Key</h2>
  {#if $account$ && publicKey && allowance}
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
  {:else if $account$ === undefined}
    <ProgressSpinner />
  {:else}
    <MessageBox type="info">
      Please log in to generate a new access key!
    </MessageBox>
  {/if}
</div>

<style lang="scss">
  .page {
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }

  h2 {
    text-align: center;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    border-bottom: 1px solid var(--color-border);
    padding: 0.4rem 0;
  }
</style>
