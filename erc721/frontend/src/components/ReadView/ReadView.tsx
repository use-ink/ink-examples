import { useState } from 'react';
import { Button, InputField, NumberInput } from 'ui';
import { ChainContract, useCall } from 'useink';
import { pickDecoded, pickError } from 'useink/utils';

interface Props {
  erc721: ChainContract;
}

export const ReadView: React.FC<Props> = ({ erc721 }) => {
  const ownerOf = useCall<string>(erc721, 'ownerOf');
  const [ownerOfId, setOwnerOfId] = useState(1);

  const balanceOf = useCall<number>(erc721, 'balanceOf');
  const [balanceOfOwner, setBalOfOwner] = useState('');

  const getApproved = useCall<string>(erc721, 'getApproved');
  const [approvedTokenId, setApprovedTokenId] = useState(1);

  const isApprovedForAll = useCall<boolean>(erc721, 'isApprovedForAll');
  const [approvedForAllOwner, setApprovedForAllOwner] = useState('');
  const [approvedForAllOperator, setApprovedForAllOperator] = useState('');

  return (
    <div className='mt-6'>
      <div>
        <label className='mt-6 font-semibold uppercase text-xs'>Token ID</label>
        <NumberInput
          value={ownerOfId}
          min={1}
          max={10}
          onChange={setOwnerOfId}
          placeholder='Enter a token ID...'
          disabled={ownerOf.isSubmitting}
        />
        <Button
          className='w-full mt-3'
          disabled={ownerOf.isSubmitting || !ownerOfId}
          onClick={() => ownerOf.send([ownerOfId], { defaultCaller: true })}
        >
          Get Owner Of
        </Button>

        {pickDecoded(ownerOf.result) && (
          <h2 className='text-white font-base text-xs mt-3 text-center overflow-ellipsis'>
            {pickDecoded(ownerOf.result)}
          </h2>
        )}

        {pickError(ownerOf.result) && (
          <h2 className='text-error-500 font-bold text-xl mt-3 text-center'>
            {pickError(ownerOf.result)}
          </h2>
        )}
      </div>

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

        {balanceOf !== undefined && (
          <h2 className='text-white font-bold text-xl mt-3 text-center'>
            {pickDecoded(balanceOf.result)}
          </h2>
        )}
      </div>

      <div className='mt-6'>
        <label className='mt-6 font-semibold uppercase text-xs'>Token ID</label>
        <NumberInput
          value={approvedTokenId}
          min={1}
          max={10}
          onChange={setApprovedTokenId}
          placeholder='Enter a token ID...'
          disabled={getApproved.isSubmitting}
        />
        <Button
          className='w-full mt-3'
          disabled={getApproved.isSubmitting || !approvedTokenId}
          onClick={() =>
            getApproved.send([approvedTokenId], { defaultCaller: true })
          }
        >
          Get Approved Account
        </Button>

        {pickDecoded(getApproved.result) && (
          <h2 className='text-white font-bold text-xs mt-3 text-center'>
            {pickDecoded(getApproved.result)}
          </h2>
        )}

        {pickError(getApproved.result) && (
          <h2 className='text-error-500 font-bold text-xs mt-3 text-center'>
            {pickError(getApproved.result)}
          </h2>
        )}
      </div>

      <div className='mt-6'>
        <label className='mt-6 font-semibold uppercase text-xs'>Owner</label>
        <InputField
          value={approvedForAllOwner}
          onChange={(e) => setApprovedForAllOwner(e.target.value)}
          placeholder='Enter the owner...'
          disabled={isApprovedForAll.isSubmitting}
        />
        <label className='mt-6 font-semibold uppercase text-xs'>Operator</label>
        <InputField
          value={approvedForAllOperator}
          onChange={(e) => setApprovedForAllOperator(e.target.value)}
          placeholder='Enter the operator...'
          disabled={isApprovedForAll.isSubmitting}
        />
        <Button
          className='w-full mt-3'
          disabled={
            isApprovedForAll.isSubmitting ||
            !approvedForAllOwner ||
            !approvedForAllOperator
          }
          onClick={() =>
            isApprovedForAll.send(
              [approvedForAllOwner, approvedForAllOperator],
              { defaultCaller: true },
            )
          }
        >
          Get Approved for All
        </Button>

        {pickDecoded(isApprovedForAll.result) !== undefined && (
          <h2 className='text-white font-bold text-xl mt-3 text-center'>
            {`${pickDecoded(isApprovedForAll.result)}`}
          </h2>
        )}

        {pickError(isApprovedForAll.result) && (
          <h2 className='text-white font-bold text-xl mt-3 text-center'>
            {pickError(isApprovedForAll.result)}
          </h2>
        )}
      </div>
    </div>
  );
};
