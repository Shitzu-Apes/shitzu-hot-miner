import type { ConnectConfig, Contract } from "near-api-js";
import type { ContractMethods } from "near-api-js/lib/contract";
import { derived, readable } from "svelte/store";

import type { ContractViewCall } from "../models";

const nearConfig = {
  networkId: import.meta.env.VITE_NETWORK_ID,
  nodeUrl: import.meta.env.VITE_NODE_URL,
  walletUrl: import.meta.env.VITE_WALLET_URL,
  helperUrl: import.meta.env.VITE_HELPER_URL,
} as const satisfies ConnectConfig;

export const near$ = readable(
  import("near-api-js").then(({ connect, Account, Contract }) =>
    connect(nearConfig).then((near) => [near, Account, Contract] as const),
  ),
);

export const shitzuTokenContract$ = derived(near$, async (n) => {
  const [near, Account, Contract] = await n;
  const account = new Account(
    near.connection,
    import.meta.env.VITE_SHITZU_TOKEN_ID,
  );
  return new Contract(account, import.meta.env.VITE_SHITZU_TOKEN_ID, {
    viewMethods: ["storage_balance_of", "ft_balance_of", "ft_metadata"],
    changeMethods: [],
    useLocalViewExecution: false,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } satisfies ContractMethods as any) as FtContract;
});

export const shitzuHotContract$ = derived(near$, async (n) => {
  const [near, Account, Contract] = await n;
  const account = new Account(
    near.connection,
    import.meta.env.VITE_SHITZU_HOT_ID,
  );
  return new Contract(account, import.meta.env.VITE_SHITZU_HOT_ID, {
    viewMethods: [
      "storage_balance_of",
      "storage_balance_bounds",
      "get_balance",
    ],
    changeMethods: [],
    useLocalViewExecution: false,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } satisfies ContractMethods as any) as ShitzuHotContract;
});

export interface FtContract extends Contract {
  storage_balance_of: ContractViewCall<
    {
      account_id: string;
    },
    string | null
  >;
  ft_balance_of: ContractViewCall<
    {
      account_id: string;
    },
    string
  >;
  ft_metadata: ContractViewCall<undefined, FtMetadata>;
}

export interface FtMetadata {
  name: string;
  symbol: string;
  icon: string;
  decimals: number;
}

export interface ShitzuHotContract extends Contract {
  storage_balance_of: ContractViewCall<
    {
      account_id: string;
    },
    string | null
  >;
  storage_balance_bounds: ContractViewCall<
    unknown,
    {
      min: string;
    }
  >;
  get_balance: ContractViewCall<
    {
      account_id: string;
    },
    string | null
  >;
}

export * from "./wallet";
