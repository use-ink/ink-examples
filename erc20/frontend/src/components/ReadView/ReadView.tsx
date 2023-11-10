import { useMemo, useState } from 'react';
import { Button, InputField } from 'ui';
import { ChainContract, useCall, useCallSubscription } from 'useink';
import {
  pickDecoded,
  planckToDecimalFormatted,
  stringNumberToBN,
} from 'useink/utils';

const symbol = 'CLAMS';

interface Props {
  erc20: ChainContract;
}

export const ReadView: React.FC<Props> = ({ erc20 }) => {
  const balanceOf = useCall<string>(erc20, 'balanceOf');
  const [balanceOfOwner, setBalOfOwner] = useState('');
  const balanceResult = useMemo(() => {
    // Convert string to BN. e.g. `1,000,000,000` -> BN
    const stringWithCommas = pickDecoded(balanceOf.result);
    if (!stringWithCommas) return;
    return stringNumberToBN(stringWithCommas);
  }, [balanceOf.result]);

  const allowance = useCall<string>(erc20, 'allowance');
  const [allowanceOwner, setAllowanceOwner] = useState('');
  const [allowanceSpender, setAllowanceSpender] = useState('');
  const allowanceResult = useMemo(() => {
    const stringWithCommas = pickDecoded(allowance.result);
    if (!stringWithCommas) return;
    return stringNumberToBN(stringWithCommas);
  }, [allowance.result]);

  const totalSupply = useCallSubscription<string>(erc20, 'totalSupply');
  const totalSupplyResult = useMemo(() => {
    if (!totalSupply || !totalSupply?.result) return;
    const stringWithCommas = pickDecoded(totalSupply.result) || '0';
    return stringNumberToBN(stringWithCommas);
  }, [totalSupply.result]);

  return (
    <div className='mt-6'>
      <h2 className='text-sm uppercase'>
        Total Supply:{' '}
        {totalSupplyResult
          ? planckToDecimalFormatted(totalSupplyResult, {
              api: erc20?.contract.api,
              symbol,
            })
          : '--'}
      </h2>

      <div className='mt-6'>
        <label className='mt-6 font-semibold uppercase text-xs'>Owner</label>
        <InputField
          value={balanceOfOwner}
          onChange={(e) => setBalOfOwner(e.target.value)}
          placeholder='Enter an Address...'
          disabled={balanceOf.isSubmitting}
        />
        <Button
          className='w-full mt-3'
          disabled={balanceOf.isSubmitting || !balanceOfOwner}
          onClick={() =>
            balanceOf.send([balanceOfOwner], { defaultCaller: true })
          }
        >
          Get Balance
        </Button>

        {balanceResult && (
          <h2 className='text-white font-bold text-xl mt-3 text-center'>
            {planckToDecimalFormatted(balanceResult, {
              api: erc20?.contract.api,
              symbol: 'CLAMS',
            })}
          </h2>
        )}
      </div>

      <div className='mt-6'>
        <label className='font-semibold uppercase text-xs'>Owner</label>
        <InputField
          value={allowanceOwner}
          onChange={(e) => setAllowanceOwner(e.target.value)}
          disabled={allowance.isSubmitting}
          placeholder='Enter an Address...'
        />
        <label className='mt-6 font-semibold uppercase text-xs'>Spender</label>
        <InputField
          value={allowanceSpender}
          onChange={(e) => setAllowanceSpender(e.target.value)}
          disabled={allowance.isSubmitting}
          placeholder='Enter an Address...'
        />
        <Button
          className='w-full mt-3'
          disabled={
            allowance.isSubmitting || !allowanceOwner || !allowanceSpender
          }
          onClick={() =>
            allowance.send([allowanceOwner, allowanceSpender], {
              defaultCaller: true,
            })
          }
        >
          Get Allowance
        </Button>

        {allowanceResult && (
          <h2 className='text-white font-bold text-xl mt-3 text-center'>
            {planckToDecimalFormatted(allowanceResult, {
              api: erc20?.contract.api,
              symbol: 'CLAMS',
            })}
          </h2>
        )}
      </div>
    </div>
  );
};
