import metadata from './assets/flipper.json';
import { CONTRACT_ROCOCO_ADDRESS } from './constants';
import { Button, Card, ConnectButton, InkLayout, formatContractName } from 'ui';
import { useCallSubscription, useContract, useTx, useWallet } from 'useink';
import { useTxNotifications } from 'useink/notifications';
import { pickDecoded, shouldDisable } from 'useink/utils';

function App() {
  const { account } = useWallet();
  const contract = useContract(CONTRACT_ROCOCO_ADDRESS, metadata);
  const getSub = useCallSubscription<boolean>(contract, 'get', [], {
    defaultCaller: true,
  });

  const flip = useTx(contract, 'flip');
  useTxNotifications(flip);

  return (
    <InkLayout
      className='md:py-12 md:p-6 p-4 h-screen flex items-center justify-center'
      animationSrc='https://raw.githubusercontent.com/paritytech/ink-workshop/d819d10a35b2ac3d2bff4f77a96701a527b3ad3a/frontend/public/dark-sea-creatures.json'
    >
      <Card className='mx-auto p-6 flex flex-col w-full max-w-md backdrop-blur-sm bg-opacity-70'>
        <h1 className='text-2xl font-bold'>
          {formatContractName(metadata.contract.name)}
        </h1>

        <p className='mt-6'>
          Flipped:{' '}
          <b className='uppercase'>{pickDecoded(getSub.result)?.toString()}</b>
        </p>

        {account ? (
          <Button
            disabled={shouldDisable(flip)}
            onClick={() => flip.signAndSend()}
            className='mt-6'
          >
            {shouldDisable(flip) ? 'Flipping...' : 'Flip'}
          </Button>
        ) : (
          <ConnectButton className='mt-6' />
        )}
      </Card>
    </InkLayout>
  );
}

export default App;
