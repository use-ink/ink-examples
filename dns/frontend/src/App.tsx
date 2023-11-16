import {
  UseInkathonProvider,
  useInkathon,
  SubstrateDeployment,
  useRegisteredContract,
  contractQuery,
  decodeOutput,
  contractTx,
  useBalance,
  rococo,
} from '@scio-labs/use-inkathon';
import DNS_ABI from './dns.json';
import { useRef, useState } from 'react';
import { blake2AsHex } from '@polkadot/util-crypto';

function App() {
  const { isConnected } = useInkathon();
  return (
    <div className='w-screen flex justify-center p-4'>
      <div className='max-w-2xl flex flex-col gap-4 '>
        <ConnectionState />
        {isConnected && (
          <>
            <RegisterName />
            <SetAddress />
            <GetAddress />
          </>
        )}
      </div>
    </div>
  );
}

const ConnectionState = () => {
  const { connect, disconnect, activeChain, activeAccount } = useInkathon();
  const { contract } = useRegisteredContract('dns');
  const balance = useBalance(activeAccount?.address, true);

  if (!activeAccount) {
    return (
      <div>
        <button type='button' onClick={() => (connect ? connect() : undefined)}>
          Connect
        </button>
      </div>
    );
  }

  return (
    <div className='card'>
      <div className='grid grid-cols-3 overflow-hidden'>
        <div>
          <div className='text-sm text-slate-500'>Chain</div>
          {activeChain && (
            <div className='text-lg font-semibold'>{activeChain.name}</div>
          )}
        </div>
        <div>
          <div className='text-sm text-slate-500'>Active Account</div>
          {activeAccount && (
            <div className='text-lg font-semibold'>{activeAccount.name}</div>
          )}
          <div className='text-sm text-ellipsis overflow-hidden'>
            {activeAccount.address}
          </div>
        </div>

        <div>
          <div className='text-sm text-slate-500'>Account Balance</div>
          {balance && (
            <div className='text-lg font-semibold'>
              {balance.balanceFormatted}
            </div>
          )}
        </div>
      </div>

      <div>
        <div className='text-sm'>Contract</div>
        {contract && (
          <div className='text-lg font-semibold'>{contract?.address}</div>
        )}
      </div>
      {activeAccount && (
        <button type='button' onClick={disconnect}>
          Disconnect
        </button>
      )}
    </div>
  );
};

const RegisterName = () => {
  const { api, activeAccount } = useInkathon();

  const [name, setName] = useState('');
  const nameRef = useRef<HTMLInputElement>(null);

  const { contract } = useRegisteredContract('dns');
  const {
    loading,
    error,
    result,
    execute: register,
  } = usePromise(async () => {
    if (!contract || !api || !activeAccount || !nameRef.current) return;
    setName(nameRef.current.value);
    return contractTx(api, activeAccount.address, contract, 'register', {}, [
      blake2AsHex(nameRef.current.value),
    ]);
  });

  return (
    <div className='card'>
      <div>
        <h3 className='font-semibold text-lg'>Step 1: Register a New Name</h3>
        <p className='slate-500'>
          Claim ownership of the given <code>name</code>
        </p>
      </div>

      <form className='flex flex-row gap-1' action='#'>
        <input placeholder='Name to register' ref={nameRef} />
        <button
          type='submit'
          disabled={loading}
          onClick={() => {
            register();
          }}
        >
          {loading ? 'Registering...' : 'Register'}
        </button>
      </form>

      {error && (
        <>
          <hr />
          <div className='error'>{error}</div>
        </>
      )}
      {result && !!result.successEvent && (
        <>
          <hr />
          <div className='success'>
            Name <code>{name}</code> registered!
          </div>
        </>
      )}
    </div>
  );
};

const SetAddress = () => {
  const { api, activeAccount } = useInkathon();
  const [name, setName] = useState('');
  const [accountId, setAccountId] = useState('');

  const { contract } = useRegisteredContract('dns');

  const { execute, loading, result, error } = usePromise(async () => {
    if (!contract || !api || !activeAccount || !name) return;

    return contractTx(api, activeAccount.address, contract, 'setAddress', {}, [
      blake2AsHex(name),
      accountId,
    ]);
  });

  return (
    <div className='card'>
      <div>
        <h3 id='SetAddress' className='font-semibold text-lg'>
          Step 2: Set AccountId For Name
        </h3>
        <p className='slate-500'>
          Set the <code>AccountId</code> which the given <code>name</code>{' '}
          should resolve to
        </p>
      </div>

      <form className='flex flex-col gap-1' action='#SetAddress'>
        <input
          placeholder='Name'
          onChange={(event) => setName(event.target.value)}
          id='name'
        />
        <input
          placeholder='AccountId'
          onChange={(event) => setAccountId(event.target.value)}
          id='account_id'
        />
        <button type='submit' disabled={loading} onClick={execute}>
          {loading ? 'Updating...' : 'Set'}
        </button>
      </form>

      {error && (
        <>
          <hr />
          <div className='error'>{error}</div>
        </>
      )}
      {result && !!result.successEvent && (
        <>
          <hr />
          <div className='success'>
            <div>
              Name <code>{name}</code> now resolves to:
            </div>
            <div>
              <code>{accountId}</code>
            </div>
          </div>
        </>
      )}
    </div>
  );
};

const GetAddress = () => {
  const { api } = useInkathon();
  const inputRef = useRef<HTMLInputElement>(null);
  const [name, setName] = useState('');
  const [result, setResult] = useState<null | ReturnType<typeof decodeOutput>>(
    null,
  );
  const { contract } = useRegisteredContract('dns');

  const getAddress = async (name: string) => {
    if (!contract || !api || !name) return;

    const result = await contractQuery(
      api,
      '',
      contract,
      'getAddress',
      undefined,
      [blake2AsHex(name)],
    );

    setResult(decodeOutput(result, contract, 'getAddress'));
    setName(name);
  };

  return (
    <div className='card'>
      <div>
        <h3 id='GetAddress' className='font-semibold text-lg'>
          Step 3: Lookup Name
        </h3>
        <p className='slate-500'>
          Resolves <code>Address</code> for a given <code>Name</code>
        </p>
      </div>

      <form className='flex flex-row gap-1' action={'#GetAddress'}>
        <input ref={inputRef} placeholder='Name to lookup' id='name' />
        <button
          type='submit'
          onClick={() => {
            if (inputRef.current) inputRef.current.blur();
            if (inputRef.current?.value) {
              getAddress(inputRef.current?.value);
            }
          }}
        >
          Lookup
        </button>
      </form>
      {result && (
        <>
          <hr />
          <div>
            <code className='font-bold'>{name}</code> resolves to:
          </div>
          <div>{result.decodedOutput}</div>
        </>
      )}
    </div>
  );
};

const getDeployments = async (): Promise<SubstrateDeployment[]> => {
  return [
    {
      contractId: 'dns',
      networkId: rococo.network,
      abi: DNS_ABI,
      address: '5GWCUiApMhV3QYK4RedaLpbhcCBWLeGVT2wtZPfCHhnHxoud',
    },
  ];
};

function WrappedApp() {
  return (
    <UseInkathonProvider
      appName='DNS Frontend Example'
      deployments={getDeployments()}
      defaultChain={rococo}
    >
      <App />
    </UseInkathonProvider>
  );
}

export default WrappedApp;

const usePromise = <T,>(promise: () => Promise<T>) => {
  const [result, setResult] = useState<T | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const execute = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await promise();
      setResult(result);
    } catch (error) {
      if (
        typeof error === 'object' &&
        error &&
        'errorMessage' in error &&
        typeof error.errorMessage === 'string'
      )
        setError(error.errorMessage);
      else {
        console.error(error);
        setError('Unknown error, check console');
      }
    } finally {
      setLoading(false);
    }
  };

  return { result, error, loading, execute };
};
