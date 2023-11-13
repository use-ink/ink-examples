import { useMemo, useState } from 'react';
import { Card, Tab, Tabs, formatContractName, useDeployerState } from 'ui';
import { useCallSubscription, useContract, useTx, useWallet } from 'useink';
import { useTxNotifications } from 'useink/notifications';
import {
  pickDecoded,
  planckToDecimalFormatted,
  stringNumberToBN,
} from 'useink/utils';
import metadata from '../../../assets/erc20.json';
import { ReadView } from '../ReadView';
import { WriteView } from '../WriteView';

export const Erc20: React.FC = () => {
  const { contractAddress } = useDeployerState();
  const erc20 = useContract(contractAddress || '', metadata);
  const [view, setView] = useState<'read' | 'write'>('read');

  const approve = useTx<boolean>(erc20, 'approve');
  useTxNotifications(approve);

  const { account } = useWallet();
  const balanceOf = useCallSubscription<string>(erc20, 'balanceOf', [
    account?.address || '',
  ]);
  const yourBalance = useMemo(() => {
    const stringWithCommas = pickDecoded(balanceOf.result) || '0';
    return stringNumberToBN(stringWithCommas);
  }, [balanceOf.result]);

  return (
    <div className='flex flex-col md:flex-row md:justify-center justify-center items-stretch gap-3 w-full max-h-[80%]'>
      <Card className='p-6 flex flex-col w-full max-w-md backdrop-blur-sm bg-opacity-70 overflow-y-scroll mx-auto'>
        <div className='flex items-center justify-between sm:flex-row flex-col'>
          <h1 className='text-2xl font-bold'>
            {formatContractName(metadata.contract.name)}
          </h1>

          {erc20 && account && (
            <hgroup className='text-right'>
              <h2 className='text-xs uppercase'>Your Balance</h2>
              <h2 className='text-sm uppercase font-bold'>
                {yourBalance
                  ? planckToDecimalFormatted(yourBalance, {
                      api: erc20?.contract.api,
                      symbol: 'CLAMS',
                    })
                  : '--'}
              </h2>
            </hgroup>
          )}
        </div>

        {erc20 && (
          <>
            <Tabs className='w-full mt-3'>
              <Tab onClick={() => setView('read')} isSelected={view === 'read'}>
                Read
              </Tab>
              <Tab
                onClick={() => setView('write')}
                isSelected={view === 'write'}
              >
                Write
              </Tab>
            </Tabs>

            {'read' === view ? (
              <ReadView erc20={erc20} />
            ) : (
              <WriteView erc20={erc20} />
            )}
          </>
        )}
      </Card>
    </div>
  );
};
