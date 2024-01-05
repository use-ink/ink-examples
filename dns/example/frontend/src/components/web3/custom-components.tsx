import {
    contractQuery,
    contractTx,
    decodeOutput,
    useInkathon,
    useRegisteredContract
} from '@scio-labs/use-inkathon';

import { blake2AsHex } from '@polkadot/util-crypto';
import { useRef, useState } from 'react';
import { Card, CardContent } from '@/components/ui/card'
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';


  export const RegisterName = () => {
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
      <Card>
        <CardContent className="pt-4">
        <div>
          <h3 className='font-semibold text-lg'>Step 1: Register a New Name</h3>
          <p className='slate-500'>
            Claim ownership of the given <code>name</code>
          </p>
        </div>

        <div className='flex flex-row gap-1'>
          <Input placeholder='Name to register' ref={nameRef} />
          <Button
            type='submit'
            disabled={loading}
            onClick={() => {
              register();
            }}
          >
            {loading ? 'Registering...' : 'Register'}
          </Button>
        </div>

        {error && (
          <div>
            <hr />
            <div className='error'>{error}</div>
          </div>
        )}
        {result && !!result.successEvent && (
          <>
            <hr />
            <div className='success'>
              Name <code>{name}</code> registered!
            </div>
          </>
        )}
        </CardContent>
      </Card>
    );
  };

  export const SetAddress = () => {
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
      <Card>
        <CardContent className="pt-4">
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
          <Input
            placeholder='Name'
            onChange={(event) => setName(event.target.value)}
            id='name'
          />
          <Input
            placeholder='AccountId'
            onChange={(event) => setAccountId(event.target.value)}
            id='account_id'
          />
          <Button type='submit' disabled={loading} onClick={execute}>
            {loading ? 'Updating...' : 'Set'}
          </Button>
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
      </CardContent>
      </Card>
    );
  };

  export const GetAddress = () => {
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
      <Card>
        <CardContent className="pt-4">
        <div>
          <h3 id='GetAddress' className='font-semibold text-lg'>
            Step 3: Lookup Name
          </h3>
          <p className='slate-500'>
            Resolves <code>Address</code> for a given <code>Name</code>
          </p>
        </div>

        <div className='flex flex-row gap-1'>
          <Input ref={inputRef} placeholder='Name to lookup' id='name' />
          <Button
            type='submit'
            onClick={() => {
              if (inputRef.current) inputRef.current.blur();
              if (inputRef.current?.value) {
                getAddress(inputRef.current?.value);
              }
            }}
          >
            Lookup
          </Button>
        </div>
        {result && (
          <>
            <hr />
            <div>
              <code className='font-bold'>{name}</code> resolves to:
            </div>
            <div>{result.decodedOutput}</div>
          </>
        )}
      </CardContent>
      </Card>
    );
  };


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