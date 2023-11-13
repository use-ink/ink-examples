import { useState } from 'react';
import { Card, Tab, Tabs, formatContractName, useDeployerState } from 'ui';
import { useCallSubscription, useContract, useTx, useWallet } from 'useink';
import { useTxNotifications } from 'useink/notifications';
import { pickDecoded } from 'useink/utils';
import metadata from '../../../assets/erc721.json';
import { ReadView } from '../ReadView';
import { WriteView } from '../WriteView';

export const Erc721: React.FC = () => {
  const { contractAddress } = useDeployerState();
  const erc721 = useContract(contractAddress || '', metadata);
  const [view, setView] = useState<'read' | 'write'>('read');

  const approve = useTx<boolean>(erc721, 'approve');
  useTxNotifications(approve);

  const { account } = useWallet();
  const balanceOf = useCallSubscription<number>(erc721, 'balanceOf', [
    account?.address || '',
  ]);

  return (
    <Card className='max-w-md mx-auto'>
      <div className='flex items-center justify-between sm:flex-row flex-col'>
        <h1 className='text-2xl font-bold'>
          {formatContractName(metadata.contract.name)}
        </h1>

        {erc721 && account && (
          <hgroup className='text-right'>
            <h2 className='text-xs uppercase'>Your Balance</h2>
            <h2 className='text-sm uppercase font-bold'>
              {pickDecoded(balanceOf.result) || '--'}
            </h2>
          </hgroup>
        )}
      </div>

      {erc721 && (
        <>
          <Tabs className='w-full mt-3'>
            <Tab onClick={() => setView('read')} isSelected={view === 'read'}>
              Read
            </Tab>
            <Tab onClick={() => setView('write')} isSelected={view === 'write'}>
              Write
            </Tab>
          </Tabs>

          {'read' === view ? (
            <ReadView erc721={erc721} />
          ) : (
            <WriteView erc721={erc721} />
          )}
        </>
      )}
    </Card>
  );
};
