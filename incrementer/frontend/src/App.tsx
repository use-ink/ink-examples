import { useState } from 'react';
import {
  Button,
  Card,
  ConnectButton,
  InkLayout,
  NumberInput,
  formatContractName,
} from 'ui';
import { useCallSubscription, useContract, useTx, useWallet } from 'useink';
import { useTxNotifications } from 'useink/notifications';
import { pickDecoded, shouldDisable } from 'useink/utils';
import metadata from './assets/incrementer.json';
import { CONTRACT_ROCOCO_ADDRESS } from './constants';

function App() {
  const { account } = useWallet();
  const contract = useContract(CONTRACT_ROCOCO_ADDRESS, metadata);
  const getSub = useCallSubscription<number>(contract, 'get', [], {
    defaultCaller: true,
  });
  const [incAmount, setIncAmount] = useState(1);
  const inc = useTx(contract, 'inc');
  useTxNotifications(inc);

  return (
    <InkLayout
      className='md:py-12 md:p-6 p-4 h-screen flex items-center justify-center'
      animationSrc='https://raw.githubusercontent.com/use-ink/ink-workshop/d819d10a35b2ac3d2bff4f77a96701a527b3ad3a/frontend/public/dark-sea-creatures.json'
    >
      <Card className='mx-auto p-6 flex flex-col w-full max-w-md backdrop-blur-sm bg-opacity-70'>
        <h1 className='text-2xl font-bold'>
          {formatContractName(metadata.contract.name)}
        </h1>

        <p className='my-6'>
          Current Number:{' '}
          <b className='uppercase'>{pickDecoded(getSub.result)}</b>
        </p>

        <NumberInput
          disabled={shouldDisable(inc)}
          onChange={(v: number) => setIncAmount(v)}
          value={incAmount}
          min={1}
          max={100}
        />

        {account ? (
          <Button
            disabled={shouldDisable(inc)}
            onClick={() => inc.signAndSend([incAmount])}
            className='mt-6'
          >
            {shouldDisable(inc)
              ? 'Incrementing...'
              : `Increment by ${incAmount}`}
          </Button>
        ) : (
          <ConnectButton className='mt-6' />
        )}
      </Card>
    </InkLayout>
  );
}

export default App;
