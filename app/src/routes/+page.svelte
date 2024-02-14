<script lang="ts">
  import Accordion from "@smui-extra/accordion";

  import { account$ } from "$lib/auth";
  import { AccessKey, MessageBox, Wallet } from "$lib/components";
  import ProgressSpinner from "$lib/components/ProgressSpinner.svelte";
</script>

<div class="page">
  {#if $account$}
    <Accordion multiple>
      <Wallet account={$account$} />
      <AccessKey account={$account$} />
    </Accordion>
  {:else if $account$ === undefined}
    <ProgressSpinner />
  {:else}
    <MessageBox type="info">
      Please log in to generate a new access key and to start auto mining HOT.
      You must log in with a ".tg" wallet address!
    </MessageBox>
  {/if}
</div>

<style lang="scss">
  .page {
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }
</style>
