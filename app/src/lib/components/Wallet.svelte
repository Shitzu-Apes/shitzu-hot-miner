<script lang="ts">
  import type { Account } from "@near-wallet-selector/core";
  import { FixedNumber } from "@tarnadas/fixed-number";

  import { MessageBox } from "$lib/components";
  import { Section } from "$lib/layout";
  import type { Condition } from "$lib/models";
  import { shitzuTokenContract$, type FtContract } from "$lib/near";

  export let account: Account;

  let condition: Condition;
  $: if (!account.accountId.endsWith(".tg")) {
    condition = "err";
  } else if (nearBalance != null && nearBalance.toNumber() < 1) {
    condition = "warn";
  } else {
    condition = "ok";
  }

  let nearBalance: FixedNumber | undefined;
  let shitzuBalance: FixedNumber | undefined;

  $: fetchShitzuBalance($shitzuTokenContract$);
  fetchNearBalance();

  async function fetchNearBalance() {
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
          request_type: "view_account",
          finality: "final",
          account_id: account.accountId,
        },
      }),
    });
    const json = (await res.json()) as {
      result: { amount: string; locked: string };
    };
    if (!json.result) return;
    nearBalance = new FixedNumber(json.result.amount, 24).sub(
      new FixedNumber(json.result.locked, 24),
    );
  }

  async function fetchShitzuBalance(c: Promise<FtContract>) {
    const contract = await c;
    const bal = await contract.ft_balance_of({ account_id: account.accountId });
    shitzuBalance = new FixedNumber(bal, 18);
  }
</script>

<Section header="Wallet" {condition}>
  <div class="field">
    {#if account.accountId.endsWith(".tg")}
      You successfully connected your Telegram wallet {account.accountId}
    {:else}
      <MessageBox type="warning">
        You must log in with a ".tg" wallet address! Please logout and log back
        in.
      </MessageBox>
    {/if}
  </div>
  <div class="field">
    <span>Near balance:</span>
    <span>{nearBalance ? nearBalance.format() : "-"}</span>
    {#if nearBalance != null && nearBalance.toNumber() < 1}
      <MessageBox type="warning">
        Your Near balance is low! Please top up your Near balance to not run out
        of gas.
      </MessageBox>
    {/if}
  </div>
  <div class="field">
    <span>SHITZU balance:</span>
    <span>{shitzuBalance ? shitzuBalance.format() : "-"}</span>
  </div>
</Section>
