import type { Account } from "@near-wallet-selector/core";
import { writable } from "svelte/store";
import { match } from "ts-pattern";

import Login, { showWalletSelector } from "./Login.svelte";
import WalletSelector from "./WalletSelector.svelte";

import type {
  AvailableWalletIds,
  HereWalletKeystore,
  MeteorWalletAuthKey,
} from "$lib/models";

// undefined: uninitialized
// null: not logged in
export const account$ = writable<Account | null | undefined>(undefined);

export async function getAccessKey(): Promise<string | undefined> {
  await new Promise((r) => setTimeout(r, 1_000));
  const selectedWalletId = window.localStorage.getItem(
    "near-wallet-selector:selectedWalletId",
  ) as AvailableWalletIds | null;
  if (!selectedWalletId) return;
  return match(selectedWalletId)
    .with('"here-wallet"', () => {
      const keystoreStr = window.localStorage.getItem("herewallet:keystore");
      if (!keystoreStr) return;
      const keystore = JSON.parse(keystoreStr) as HereWalletKeystore;
      return keystore.mainnet.accounts[keystore.mainnet.activeAccount];
    })
    .with('"meteor-wallet"', () => {
      const authKeyStr = window.localStorage.getItem(
        "near_app_meteor_wallet_auth_key",
      );
      if (!authKeyStr) return;
      const authKey = JSON.parse(authKeyStr) as MeteorWalletAuthKey;
      return (
        window.localStorage.getItem(
          `_meteor_wallet${authKey.accountId}:mainnet`,
        ) ?? undefined
      );
    })
    .exhaustive();
}

export { showWalletSelector, Login, WalletSelector };
