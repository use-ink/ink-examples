import { Accounts, Button, useUI } from '..';
import { Logo } from '../Logo';
import React from 'react';
import { useWallet } from 'useink';

export const Navbar: React.FC = () => {
  const { setShowConnectWallet } = useUI();
  const { account } = useWallet();

  return (
    <nav className='fixed top-0 left-0 right-0 px-6 py-4 flex items-center justify-between'>
      <figure>
        <Logo className='h-16' />
      </figure>
      <span>
        {account ? (
          <Accounts />
        ) : (
          <Button onClick={() => setShowConnectWallet(true)}>
            Connect Wallet
          </Button>
        )}
      </span>
    </nav>
  );
};
