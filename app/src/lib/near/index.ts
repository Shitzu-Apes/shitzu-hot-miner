import type { ConnectConfig, Account, Contract } from "near-api-js";
import type { ContractMethods } from "near-api-js/lib/contract";
import { readable } from "svelte/store";

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

export function createFtContract(
  account: Account,
  contract: typeof Contract,
  contractId: string,
): FtContract {
  return new contract(account, contractId, {
    viewMethods: ["storage_balance_of", "ft_balance_of", "ft_metadata"],
    changeMethods: [],
    useLocalViewExecution: false,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } satisfies ContractMethods as any) as FtContract;
}

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

export * from "./wallet";
