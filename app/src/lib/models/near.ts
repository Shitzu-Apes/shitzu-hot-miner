export type ContractViewCall<T, R> = (params: T) => Promise<R>;

export type AvailableWalletIds = '"here-wallet"' | '"meteor-wallet"';

export type HereWalletKeystore = {
  mainnet: {
    accounts: Record<string, string>;
    activeAccount: string;
  };
};

export type MeteorWalletAuthKey = {
  accountId: string;
};

export type AccessKeyReponse = {
  permission: {
    FunctionCall: {
      allowance: string;
    };
  };
};
