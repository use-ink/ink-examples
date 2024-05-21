import { useEffect, useMemo } from 'react';
import {
  Button,
  Card,
  ConnectButton,
  Events,
  InkLayout,
  formatContractName,
} from 'ui';
import { useApi, useBalance, useDeployer, useMetadata } from 'useink';
import { useContract, useTx, useWallet } from 'useink';
import { useTxNotifications } from 'useink/notifications';
import {
  formatEventName,
  isPendingSignature,
  planckToDecimalFormatted,
  shouldDisable,
} from 'useink/utils';
import metadata from './assets/contract_terminate.json';

// We already deployed the contract Wasm blob on chain. We use this code hash to
// re-deploy;
const codeHash =
  '0xf667f6dbcc27e4b0f2e23c2af12d6e5dee358910753b02910a6aebd3768dd618';

function App() {
  const { account } = useWallet();
  const chainApi = useApi();

  const M = useMetadata({ requireWasm: false }, metadata);
  const balance = useBalance(account);

  const D = useDeployer();
  useTxNotifications(D);

  const contract = useContract(D.contractAddress || '', metadata);
  const terminate = useTx(contract, 'terminateMe');
  useTxNotifications(terminate);

  useEffect(() => {
    chainApi?.api && M.abi && D.dryRun(M.abi, 'new', undefined, { codeHash });
  }, [M.abi, chainApi?.api]);

  const wasTerminated = useMemo(() => {
    return Boolean(
      terminate.events.find(
        (event) => formatEventName(event) === 'contracts:Terminated',
      ),
    );
  }, [terminate.events]);

  return (
    <InkLayout
      className='md:py-12 md:p-6 p-4 h-screen flex items-center justify-center'
      animationSrc='https://raw.githubusercontent.com/use-ink/ink-workshop/d819d10a35b2ac3d2bff4f77a96701a527b3ad3a/frontend/public/dark-sea-creatures.json'
    >
      <Card className='mx-auto p-6 flex flex-col w-full max-w-xl backdrop-blur-sm bg-opacity-70'>
        <h1 className='text-2xl font-bold'>
          {formatContractName(metadata.contract.name)}
        </h1>

        <h2 className='text-xs'>
          Your Balance:{' '}
          {planckToDecimalFormatted(balance?.freeBalance, chainApi?.api)}
        </h2>

        {D.storageDeposit && (
          <div className='mt-6 w-full text-xs'>
            <h3 className='uppercase font-semibold'>Dry Run Results</h3>

            {D.contractAddress && (
              <hgroup>
                <h4>Contract Address</h4>
                <h3>{D.contractAddress}</h3>
              </hgroup>
            )}

            {D.gasConsumed && (
              <div className='mt-3'>
                <h3 className='uppercase font-semibold'>Gas Consumed</h3>
                <ul className='p-0 list-none'>
                  <li>refTime: {D.gasConsumed.refTime.toString()}</li>
                  <li>proof size: {D.gasConsumed.proofSize.toString()}</li>
                </ul>
              </div>
            )}

            {D.gasRequired && (
              <div className='mt-3'>
                <h3 className='uppercase font-semibold'>Gas Required</h3>
                <ul className='p-0 list-none'>
                  <li>refTime: {D.gasRequired.refTime.toString()}</li>
                  <li>proof size: {D.gasRequired.proofSize.toString()}</li>
                </ul>
              </div>
            )}

            {D.storageDeposit && (
              <div className='mt-3'>
                <h3>
                  Storage Deposit:{' '}
                  {planckToDecimalFormatted(
                    D.storageDeposit.asCharge,
                    chainApi?.api,
                  )}
                </h3>
              </div>
            )}
          </div>
        )}

        {M.error && (
          <p className='text-warning-500'>
            <span className='text-xs'>Metadata: </span>
            {M.error}
          </p>
        )}

        {D.error && <p className='text-warning-500'>{D.error}</p>}

        {!D.wasDeployed ? (
          <div className='mt-6'>
            <h2 className='font-bold text-xl'>
              {D.storageDeposit
                ? "Let's first deploy the contract!"
                : 'Loading...'}
            </h2>

            {account ? (
              <Button
                disabled={shouldDisable(D) || !D.willBeSuccessful}
                onClick={() =>
                  M.abi && D.signAndSend(M.abi, 'new', undefined, { codeHash })
                }
                className='w-full mt-6'
              >
                {isPendingSignature(D)
                  ? 'Please sign transaction...'
                  : shouldDisable(D)
                  ? 'Deploying...'
                  : 'Deploy'}
              </Button>
            ) : (
              <ConnectButton className='w-full mt-6' />
            )}
          </div>
        ) : (
          <div className='mt-6'>
            <h2 className='font-bold text-xl'>
              {wasTerminated
                ? 'Your contract was terminated!'
                : 'Your contract has been deployed!'}
            </h2>
            {account ? (
              wasTerminated ? (
                <Button
                  onClick={() => {
                    D.resetState();
                    terminate.resetState();
                    D.dryRun(M.abi, 'new', undefined, { codeHash });
                  }}
                  className='w-full mt-6'
                >
                  Try it again
                </Button>
              ) : (
                <Button
                  disabled={shouldDisable(terminate) || wasTerminated}
                  onClick={() => terminate.signAndSend()}
                  className='w-full mt-6'
                >
                  {isPendingSignature(terminate)
                    ? 'Sign transaction...'
                    : shouldDisable(terminate)
                    ? 'Terminating...'
                    : 'Terminate'}
                </Button>
              )
            ) : (
              <ConnectButton className='w-full mt-6' />
            )}
          </div>
        )}

        <Events className='mt-6' events={terminate.events} />
      </Card>
    </InkLayout>
  );
}

export default App;
