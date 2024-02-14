<script lang="ts">
  import type { Account, WalletSelector } from "@near-wallet-selector/core";
  import Button from "@smui/button";
  import { FixedNumber } from "@tarnadas/fixed-number";
  import { writable } from "svelte/store";
  import { bind } from "svelte-simple-modal";

  import { MessageBox, ShitzuDeposit } from "$lib/components";
  import { ModalSize, Section, modal$, modalSize$ } from "$lib/layout";
  import type { Condition } from "$lib/models";
  import {
    type ShitzuHotContract,
    shitzuHotContract$,
    selector$,
    type FtContract,
    shitzuTokenContract$,
  } from "$lib/near";
  import { showTxSnackbar } from "$lib/snackbar";

  export let account: Account;

  let loading = false;
  let isRegistered: boolean | undefined;
  let shitzuBalance$ = writable<FixedNumber>();
  let depositBalance$ = writable<FixedNumber>();

  let condition: Condition;
  $: if (isRegistered == null || $depositBalance$ == null) {
    condition = "loading";
  } else if (!isRegistered || $depositBalance$.toNumber() === 0) {
    condition = "err";
  } else if ($depositBalance$.toNumber() < 200) {
    condition = "warn";
  } else {
    condition = "ok";
  }

  $: fetchIsRegistered($shitzuHotContract$);
  $: fetchShitzuBalance($shitzuTokenContract$);
  $: fetchDepositBalance($shitzuHotContract$);

  async function fetchIsRegistered(c: Promise<ShitzuHotContract>) {
    const contract = await c;
    const res = await contract.storage_balance_of({
      account_id: account.accountId,
    });
    isRegistered = !!res;
  }

  async function fetchShitzuBalance(c: Promise<FtContract>) {
    const contract = await c;
    const bal = await contract.ft_balance_of({ account_id: account.accountId });
    $shitzuBalance$ = new FixedNumber(bal, 18);
  }

  async function fetchDepositBalance(c: Promise<ShitzuHotContract>) {
    const contract = await c;
    const bal = await contract.get_balance({ account_id: account.accountId });
    $depositBalance$ = new FixedNumber(bal, 18);
  }

  async function registerAccount(s: Promise<WalletSelector>) {
    const selector = await s;
    const wallet = await selector.wallet();
    const txPromise = wallet.signAndSendTransaction({
      receiverId: import.meta.env.VITE_SHITZU_HOT_ID,
      actions: [
        {
          type: "FunctionCall",
          params: {
            methodName: "storage_deposit",
            args: {
              account_id: account.accountId,
            },
            gas: "30000000000000",
            deposit: new FixedNumber(10_000_000_000_000_000_000_000n).toU128(),
          },
        },
      ],
    });
    loading = true;
    showTxSnackbar(txPromise);
    txPromise
      .then(() => {
        fetchIsRegistered($shitzuHotContract$);
      })
      .finally(() => {
        loading = false;
      });
  }

  async function openDepositModal() {
    modalSize$.set(ModalSize.Small);
    modal$.set(
      bind(ShitzuDeposit, {
        shitzuBalance$,
        depositBalance$,
        afterUpdateBalances: () => {
          fetchShitzuBalance($shitzuTokenContract$);
          fetchDepositBalance($shitzuHotContract$);
        },
      }),
    );
  }
</script>

<Section header="Account" {condition}>
  <div class="field">
    {#if isRegistered === false}
      <Button
        disabled={loading}
        variant="outlined"
        on:click={() => registerAccount($selector$)}
        style="align-self: center;"
      >
        Register Account
      </Button>
      <MessageBox type="info">
        You need to register your TG wallet in our smart contract. This incurs a
        small deposit fee of 0.1N to cover storage cost.
      </MessageBox>
    {:else if isRegistered === true}
      Your account is successfully registered in our smart contract
    {/if}
  </div>
  <div class="field">
    <span>Shitzu balance (wallet):</span>
    <span>{$shitzuBalance$ ? $shitzuBalance$.format() : "-"}</span>
    <Button
      variant="outlined"
      href="https://app.ref.finance/#token.0xshitzu.near|near"
      target="_blank"
      rel="noopener"
      style="align-self: center;"
    >
      Buy SHITZU
    </Button>
  </div>
  <div class="field">
    <span>Shitzu balance (deposit):</span>
    <span>{$depositBalance$ ? $depositBalance$.format() : "-"}</span>
    {#if $depositBalance$ != null}
      {#if $depositBalance$.toNumber() === 0}
        <MessageBox type="error">
          You need to deposit SHITZU token in order to auto mine HOT!
        </MessageBox>
      {:else if $depositBalance$.toNumber() < 200}
        <MessageBox type="warning">
          Your deposited SHITZU balance is very low! Please deposit more to
          continue auto mining HOT.
        </MessageBox>
      {/if}
    {/if}
    <Button
      disabled={loading || !$shitzuBalance$ || !$depositBalance$}
      variant="outlined"
      on:click={openDepositModal}
      style="align-self: center;"
    >
      Deposit / Withdraw
    </Button>
  </div>
</Section>
