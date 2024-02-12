import { Hono } from 'hono';

import type { Env } from './global';

export type Account = {
  accountId: string;
  functionAccessKey?: string;
};

export const accounts = new Hono<{ Bindings: Env }>().get('/', async c => {
  const addr = c.env.INFO.idFromName('');
  const obj = c.env.INFO.get(addr);
  const res = await obj.fetch(c.req.url);
  const info = await res.json<Account>();
  return c.json(info);
});

export class Accounts {
  private state: DurableObjectState;
  private app: Hono<{ Bindings: Env }>;
  private accountIds: string[];
  private accounts: Record<string, Account>;

  constructor(state: DurableObjectState) {
    this.state = state;
    this.accountIds = [];
    this.accounts = {};
    this.state.blockConcurrencyWhile(async () => {
      const accountIds = await this.state.storage.get<string[]>('accountIds');
      this.accountIds = accountIds ?? [];
    });

    this.app = new Hono();
    this.app
      .get('/:account_id', async c => {
        const accountId = c.req.param('account_id');
        if (this.accounts[accountId]) {
          return c.json(this.accounts[accountId]);
        }
        if (this.accountIds.includes(accountId)) {
          const account = await this.state.storage.get<Account>(
            `account:${accountId}`
          );
          if (!account) {
            return new Response(null, { status: 500 });
          }
          this.accounts[accountId] = account;
          return c.json(account);
        }
        return new Response(null, { status: 404 });
      })
      .post('/:account_id/add_access_key', async () => {
        // TODO
        return new Response(null, { status: 501 });
      });
  }

  async fetch(request: Request): Promise<Response> {
    return this.app.fetch(request);
  }
}
