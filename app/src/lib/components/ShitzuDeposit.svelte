<script lang="ts">
  import type { WalletSelector } from "@near-wallet-selector/core";
  import Button from "@smui/button";
  import Tab, { Label } from "@smui/tab";
  import TabBar from "@smui/tab-bar";
  import { FixedNumber } from "@tarnadas/fixed-number";
  import { writable, type Writable } from "svelte/store";

  import { TokenInput } from "$lib/components";
  import { ModalContent } from "$lib/layout";
  import { selector$ } from "$lib/near";
  import { showTxSnackbar } from "$lib/snackbar";

  export let shitzuBalance$: Writable<FixedNumber>;
  export let depositBalance$: Writable<FixedNumber>;
  export let afterUpdateBalances: () => void;

  let newShitzuBalance: FixedNumber | undefined;
  let newDepositBalance: FixedNumber | undefined;

  let tabs = [
    {
      label: "Deposit",
    },
    {
      label: "Withdraw",
    },
  ];
  let active = tabs[0];

  let input: TokenInput;
  let inputValue$ = writable<string | undefined>();
  $: input$ = input?.u128$;

  $: if (active) {
    $inputValue$ = "";
  }

  $: updateOutAmount(active.label, $input$);
  function updateOutAmount(label: string, val?: FixedNumber) {
    if (!val || val.valueOf() === 0n) {
      newShitzuBalance = undefined;
      newDepositBalance = undefined;
      return;
    }
    if (label === "Deposit") {
      newShitzuBalance = $shitzuBalance$.sub(val);
      newDepositBalance = $depositBalance$.add(val);
    } else {
      newShitzuBalance = $shitzuBalance$.add(val);
      newDepositBalance = $depositBalance$.sub(val);
    }
  }

  let disabled = true;
  $: if (!newShitzuBalance || !newDepositBalance) {
    disabled = true;
  } else if (
    newShitzuBalance.valueOf() < 0 ||
    newDepositBalance.valueOf() < 0
  ) {
    disabled = true;
  } else if ($input$?.valueOf() === 0n) {
    disabled = true;
  } else {
    disabled = false;
  }

  function setMax() {
    if (active.label === "Deposit") {
      $inputValue$ = $shitzuBalance$.format({
        maximumSignificantDigits: 20,
        useGrouping: false,
      });
    } else {
      $inputValue$ = $depositBalance$.format({
        maximumSignificantDigits: 20,
        useGrouping: false,
      });
    }
  }

  let loading = false;
  async function runTx(s: Promise<WalletSelector>) {
    if (!$input$ || $input$.valueOf() === 0n) return;
    const selector = await s;
    const wallet = await selector.wallet();
    const isDeposit = active.label === "Deposit";
    const txPromise = wallet.signAndSendTransaction({
      receiverId: isDeposit
        ? import.meta.env.VITE_SHITZU_TOKEN_ID
        : import.meta.env.VITE_SHITZU_HOT_ID,
      actions: [
        {
          type: "FunctionCall",
          params: isDeposit
            ? {
                methodName: "ft_transfer_call",
                args: {
                  receiver_id: import.meta.env.VITE_SHITZU_HOT_ID,
                  amount: $input$.toU128(),
                  msg: JSON.stringify("Deposit"),
                },
                gas: "50000000000000",
                deposit: "1",
              }
            : {
                methodName: "withdraw",
                args: {
                  amount: $input$.toU128(),
                },
                gas: "50000000000000",
                deposit: "0",
              },
        },
      ],
    });
    loading = true;
    showTxSnackbar(txPromise);
    txPromise
      .then(() => {
        afterUpdateBalances();
        $inputValue$ = "";
      })
      .finally(() => {
        loading = false;
      });
  }
</script>

<ModalContent header="Deposit / Withdraw">
  <TabBar {tabs} let:tab bind:active>
    <Tab {tab}>
      <Label>{tab.label}</Label>
    </Tab>
  </TabBar>

  <div class="tab">
    <div class="field">
      <span>Shitzu balance (wallet):</span>
      <div class="balance">
        <span>{$shitzuBalance$.format()}</span>
        {#if newShitzuBalance}
          <span>⇒</span>
          <span
            class="result"
            class:more={newShitzuBalance > $shitzuBalance$}
            class:less={newShitzuBalance < $shitzuBalance$}
          >
            {newShitzuBalance.format()}
          </span>
        {/if}
      </div>
    </div>
    <div class="field">
      <span>Shitzu balance (deposit):</span>
      <div class="balance">
        <span>{$depositBalance$.format()}</span>
        {#if newDepositBalance}
          <span>⇒</span>
          <span
            class="result"
            class:more={newDepositBalance > $depositBalance$}
            class:less={newDepositBalance < $depositBalance$}
          >
            {newDepositBalance.format()}
          </span>
        {/if}
      </div>
    </div>

    <div class="input-wrapper">
      <TokenInput
        bind:this={input}
        bind:value={$inputValue$}
        decimals={18}
        --width="0"
        --flex="1 0 auto"
      />
      <Button variant="outlined" class="button-small" on:click={setMax}>
        max
      </Button>
    </div>

    <Button
      variant="outlined"
      on:click={() => runTx($selector$)}
      disabled={disabled || loading}>{active.label}</Button
    >
  </div>
</ModalContent>

<style lang="scss">
  .tab {
    display: flex;
    flex-direction: column;
    margin-top: 1rem;
    gap: 0.4rem;
  }

  .input-wrapper {
    display: flex;
    align-items: center;
    border: to-rem(2px) solid lightgray;
    border-radius: 0.4rem;
    padding: 0.4rem;
    max-width: 100%;

    &:hover {
      border-color: var(--bright-blue);
    }
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    padding: 0.8rem 0 0.3rem;

    &:first-child {
      padding-top: 0;
    }
    &:not(:last-child) {
      border-bottom: 1px solid var(--color-border);
    }
  }

  .balance {
    display: flex;
    align-items: center;

    > *:first-child,
    > *:last-child {
      flex: 1 1 5rem;
    }

    .result {
      text-align: end;
    }

    span {
      &.more {
        color: var(--color-ok);
      }
      &.less {
        color: var(--color-err);
      }
    }
  }

  :global(.button-small) {
    margin-left: 0.2rem;
    height: 1.6rem;
  }
</style>
