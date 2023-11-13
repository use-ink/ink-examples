import React from 'react';
import { useInstalledWallets, useUninstalledWallets, useWallet } from 'useink';
import { Button } from '../Button';

export const ConnectWallet: React.FC = () => {
  const { account, connect } = useWallet();
  const installed = useInstalledWallets();
  const uninstalled = useUninstalledWallets();

  if (account) return null;

  return (
    <div>
      <h2 className='text-2xl font-bold mb-8 text-white'>Connect Wallet</h2>

      {!account && installed.length > 0 && (
        <ul>
          {installed.map((w) => (
            <li key={w.title} className='mt-3'>
              <Button
                className='flex items-center gap-2 w-full'
                onClick={() => {
                  connect(w.extensionName);
                }}
              >
                <img className='w-10 mr-2' src={w.logo.src} alt={w.logo.alt} />
                Connect to {w.extensionName}
              </Button>
            </li>
          ))}
        </ul>
      )}

      {!account && uninstalled.length && installed.length === 0 && (
        <>
          <p className='font-semibold my-6 text-center text-white/80'>
            Please install one of these supported wallets.
          </p>

          <ul>
            {uninstalled.map((w) => (
              <li key={w.title} className='mt-3'>
                <Button
                  className='flex items-center gap-2 w-full'
                  onClick={() => window.open(w.installUrl, '_blank')}
                  rel='noopener noreferrer'
                >
                  <img
                    className='w-10 mr-2'
                    src={w.logo.src}
                    alt={w.logo.alt}
                  />
                  Install {w.extensionName}
                </Button>
              </li>
            ))}
          </ul>
        </>
      )}
    </div>
  );
};
