import { Card } from 'ui';
import { useCallSubscription, useContract } from 'useink';
import { pickDecoded } from 'useink/utils';
import otherContractMetadata from './assets/other_contract.json';

interface Props {
  address: string;
}

export const Other: React.FC<Props> = ({ address }) => {
  const otherContract = useContract(address, otherContractMetadata);
  const getOtherSub = useCallSubscription<boolean>(otherContract, 'get', [], {
    defaultCaller: true,
  });

  return (
    <Card className='p-6 flex flex-col w-full max-w-md backdrop-blur-sm bg-opacity-70'>
      <h1 className='text-2xl font-bold'>
        {otherContractMetadata.contract.name.toUpperCase()}
      </h1>

      <p className='mt-6'>
        Flipped:{' '}
        <b className='uppercase'>
          {pickDecoded(getOtherSub.result)?.toString()}
        </b>
      </p>
    </Card>
  );
};
