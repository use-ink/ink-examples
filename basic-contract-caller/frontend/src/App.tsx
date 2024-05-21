import { useEffect } from 'react';
import { Button, Card, ConnectButton, InkLayout, formatContractName } from 'ui';
import { useCall, useContract, useTx, useWallet } from 'useink';
import { useTxNotifications } from 'useink/notifications';
import { isPendingSignature, pickDecoded, shouldDisable } from 'useink/utils';
import { Other } from './Other';
import metadata from './assets/basic_contract_caller.json';
import { BASIC_CONTRACT_ROCOCO_ADDRESS } from './constants';

function App() {
  const { account } = useWallet();
  const basicContract = useContract(BASIC_CONTRACT_ROCOCO_ADDRESS, metadata);
  const flipAndGet = useTx<boolean>(basicContract, 'flipAndGet');
  useTxNotifications(flipAndGet);

  const other = useCall<string>(basicContract, 'other');

  useEffect(() => {
    other?.send([], { defaultCaller: true });
  }, [other.send]);

  const otherAddress = pickDecoded(other.result);

  return (
    <InkLayout
      className='md:py-12 md:p-6 p-4 h-screen flex items-center justify-center'
      animationSrc='https://raw.githubusercontent.com/use-ink/ink-workshop/d819d10a35b2ac3d2bff4f77a96701a527b3ad3a/frontend/public/dark-sea-creatures.json'
    >
      <div className='flex flex-col justify-center items-center gap-3 h-full'>
        <Card className='p-6 flex flex-col w-full max-w-md backdrop-blur-sm bg-opacity-70'>
          <h1 className='text-2xl font-bold'>
            {formatContractName(metadata.contract.name)}
          </h1>

          {account ? (
            <Button
              disabled={shouldDisable(flipAndGet)}
              onClick={() => flipAndGet.signAndSend()}
              className='mt-6'
            >
              {isPendingSignature(flipAndGet)
                ? 'Please sign transaction...'
                : shouldDisable(flipAndGet)
                ? 'Flipping...'
                : 'Flip Other Contract'}
            </Button>
          ) : (
            <ConnectButton className='mt-6' />
          )}
        </Card>

        {otherAddress && <Other address={otherAddress} />}
      </div>
    </InkLayout>
  );
}

export default App;
