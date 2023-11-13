import React from 'react';
import { useWallet } from 'useink';
import { Accounts, Button, ConnectWallet, useUI } from '../..';

export const ManageWallet: React.FC = () => {
  const { account, disconnect } = useWallet();
  const { setView } = useUI();

  return (
    <>
      <ConnectWallet />
      <Accounts />

      {account && (
        <>
          <Button
            className='mx-auto mt-6 w-full'
            onClick={() => setView('contract')}
          >
            View Contract
          </Button>

          <Button className='mx-auto mt-6 w-full' onClick={disconnect}>
            Disconnect
          </Button>
        </>
      )}
    </>
  );
};
