import React, {
  PropsWithChildren,
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
} from 'react';
import { useApi, useDeployer, useMetadata, useWallet } from 'useink';
import { useTxNotifications } from 'useink/notifications';
import {
  isFinalized,
  isPendingSignature,
  shouldDisableStrict,
} from 'useink/utils';
import { RunResults } from '../..';
import { Button } from '../../Button';
import { Card } from '../../Card';
import { ConnectButton } from '../../ConnectButton';
import { formatContractName } from '../../utils';

export interface DeployerContextProps {
  metadata: Record<string, unknown>;
  constructorArgs: Record<string, unknown> | undefined;
  constructorName: string;
  codeHash: string;
}

export interface DeployerState {
  contractAddress?: string;
  clearContract: () => void;
}

const DeployerContext = createContext<DeployerState>({
  clearContract: () => null,
});

function getContractAddress(key: string): string | null {
  return localStorage.getItem(key);
}

function setContractAddress(key: string, address: string) {
  localStorage.setItem(key, address);
}

function removeContractAddress(key: string) {
  if (getContractAddress(key)) localStorage.removeItem(key);
}

export const DeployerProvider: React.FC<
  PropsWithChildren<DeployerContextProps>
> = ({ children, metadata, constructorArgs, constructorName, codeHash }) => {
  const chainApi = useApi();
  const { account } = useWallet();
  const M = useMetadata({ requireWasm: false }, metadata);
  const D = useDeployer();
  useTxNotifications(D);

  const name = useMemo(
    () => (metadata?.contract as { name?: string })?.name || '',
    [metadata],
  );
  const storageKey = name ? `${name}-address` : '';

  const savedAddress = useMemo(() => getContractAddress(storageKey), [name]);
  const clearContract = useCallback(() => {
    removeContractAddress(storageKey);
  }, [name]);

  const signAndSend = useCallback(() => {
    D.signAndSend(M.abi, constructorName, constructorArgs, { codeHash });
  }, [M.abi]);

  useEffect(() => {
    chainApi?.api &&
      M.abi &&
      D.dryRun(M.abi, constructorName, constructorArgs, {
        codeHash,
        defaultCaller: true,
      });
  }, [M.abi, chainApi?.api]);

  useEffect(() => {
    if (D.contractAddress && D.wasDeployed && isFinalized(D)) {
      setContractAddress(storageKey, D.contractAddress);
    }
  }, [D.status]);

  return (
    <DeployerContext.Provider
      value={{
        contractAddress: savedAddress
          ? savedAddress
          : D.wasDeployed && isFinalized(D)
          ? D.contractAddress
          : undefined,
        clearContract,
      }}
    >
      {savedAddress || (D.wasDeployed && isFinalized(D)) ? (
        children
      ) : (
        <div className='flex flex-col md:flex-row md:justify-center justify-start items-stretch gap-3 w-full'>
          <Card className='p-6 flex flex-col w-full max-w-md backdrop-blur-sm bg-opacity-70'>
            <div className='mt-6'>
              <hgroup>
                <h1 className='text-2xl font-bold'>
                  {formatContractName(name)}
                </h1>
                <h2 className='font-bold text-xl'>
                  {D.storageDeposit
                    ? "Let's first deploy the contract!"
                    : 'Loading...'}
                </h2>
              </hgroup>

              {D.storageDeposit && (
                <RunResults
                  className='mt-6 w-full text-xs'
                  {...{ ...D, chainApi }}
                />
              )}

              {M.error && (
                <p className='text-warning-500 mt-3'>
                  <span className='text-xs'>Metadata: </span>
                  {M.error}
                </p>
              )}

              {account && D.error && (
                <p className='text-warning-500 mt-3'>
                  <span className='text-xs'>Deployer: </span>
                  {D.error}
                </p>
              )}

              {account ? (
                <Button
                  disabled={shouldDisableStrict(D) || !D.willBeSuccessful}
                  onClick={signAndSend}
                  className='w-full mt-6'
                >
                  {isPendingSignature(D)
                    ? 'Please sign transaction...'
                    : shouldDisableStrict(D)
                    ? 'Deploying...'
                    : 'Deploy'}
                </Button>
              ) : (
                <ConnectButton className='w-full mt-6' />
              )}
            </div>
          </Card>
        </div>
      )}
    </DeployerContext.Provider>
  );
};

export const useDeployerState = () => useContext(DeployerContext);
