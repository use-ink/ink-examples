import { useMemo, useState } from 'react';
import {
  Button,
  Card,
  ConnectButton,
  InkLayout,
  Link,
  NumberInput,
  formatContractName,
} from 'ui';
import { useBalance, useContract, useTx, useWallet } from 'useink';
import { useTxNotifications } from 'useink/notifications';
import {
  decimalToPlanck,
  isPendingSignature,
  planckToDecimal,
  planckToDecimalFormatted,
  shouldDisable,
  stringNumberToBN,
} from 'useink/utils';
import metadata from './assets/contract_transfer.json';
import { CONTRACT_ROCOCO_ADDRESS } from './constants';

function App() {
  const { account } = useWallet();
  const chainContract = useContract(CONTRACT_ROCOCO_ADDRESS, metadata);
  const contractBalance = useBalance({ address: CONTRACT_ROCOCO_ADDRESS });
  const userBalance = useBalance(account);
  const [amount, setAmount] = useState(1);
  const giveMe = useTx(chainContract, 'giveMe');
  useTxNotifications(giveMe);

  const planckAmount = useMemo(
    () => decimalToPlanck(amount, chainContract?.contract?.api) || 0,
    [chainContract?.contract.api, amount],
  );

  const needsMoreFunds = useMemo(
    () =>
      contractBalance?.freeBalance.lt(
        stringNumberToBN(planckAmount?.toString() || '0'),
      ),
    [contractBalance?.freeBalance, planckAmount],
  );

  return (
    <InkLayout
      className='md:py-12 md:p-6 p-4 h-screen flex items-center justify-center'
      animationSrc='https://raw.githubusercontent.com/use-ink/ink-workshop/d819d10a35b2ac3d2bff4f77a96701a527b3ad3a/frontend/public/dark-sea-creatures.json'
    >
      <div className='flex flex-col justify-center items-center h-full'>
        <Card className='mx-auto p-6 flex flex-col w-full max-w-md backdrop-blur-sm bg-opacity-70'>
          <h1 className='text-2xl font-bold'>
            {formatContractName(metadata.contract.name)}
          </h1>

          <hgroup className='mt-6 mb-2 ml-2 text-white/80 text-xs'>
            <h3>
              Contract Balance:{' '}
              <b className='uppercase'>
                {contractBalance
                  ? planckToDecimalFormatted(contractBalance?.freeBalance, {
                      api: chainContract?.contract.api,
                      significantFigures: 4,
                    })
                  : '--'}
              </b>
            </h3>

            <h3>
              Your Balance:{' '}
              <b className='uppercase'>
                {userBalance
                  ? planckToDecimalFormatted(userBalance?.freeBalance, {
                      api: chainContract?.contract.api,
                      significantFigures: 4,
                    })
                  : '--'}
              </b>
            </h3>
          </hgroup>

          <NumberInput
            disabled={shouldDisable(giveMe)}
            onChange={(v: number) => setAmount(v)}
            value={amount}
            min={1}
            max={Math.floor(
              planckToDecimal(contractBalance?.freeBalance, {
                api: chainContract?.contract.api,
              }) || 0,
            )}
          />

          {account ? (
            <Button
              disabled={shouldDisable(giveMe) || needsMoreFunds}
              onClick={() => giveMe.signAndSend([planckAmount])}
              className='mt-6'
            >
              {isPendingSignature(giveMe)
                ? 'Please sign transaction...'
                : shouldDisable(giveMe)
                ? `Sending you ${amount} ROC...`
                : `Withdraw ${amount} ROC`}
            </Button>
          ) : (
            <ConnectButton className='mt-6' />
          )}

          <div className='text-center mt-6'>
            {needsMoreFunds && (
              <p className='mb-3'>There are not enough funds.</p>
            )}

            <Link
              href={`https://use.ink/faucet?acc=${CONTRACT_ROCOCO_ADDRESS}`}
              target='_blank'
            >
              Add ROC to contract with faucet
            </Link>
          </div>
        </Card>
      </div>
    </InkLayout>
  );
}

export default App;
