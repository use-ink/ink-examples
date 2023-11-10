import { useState } from 'react';
import { BigIntInputField, Button, ConnectButton, InputField, Label } from 'ui';
import { ChainContract, useTx, useWallet } from 'useink';
import { useTxNotifications } from 'useink/notifications';
import {
  isPendingSignature,
  planckToDecimalFormatted,
  shouldDisable,
} from 'useink/utils';

interface Props {
  erc20: ChainContract;
}

export const WriteView: React.FC<Props> = ({ erc20 }) => {
  const { account } = useWallet();

  const [transferAmount, setTransferAmount] = useState(0n);
  const [transferToAddress, setTransferToAddress] = useState('');
  const transfer = useTx<null>(erc20, 'transfer');
  useTxNotifications(transfer);

  const [approveAmount, setApproveAmount] = useState(0n);
  const [approveSpender, setApproveSpender] = useState('');
  const approve = useTx<null>(erc20, 'approve');
  useTxNotifications(approve);

  const [transferFromAmount, setTransferFromAmount] = useState(0n);
  const [transferFromAddress, setTransferFromAddress] = useState('');
  const [transferFromToAddress, setTransferFromToAddress] = useState('');
  const transferFrom = useTx<null>(erc20, 'transferFrom');
  useTxNotifications(transferFrom);

  if (!account) return <ConnectButton className='mt-6 mx-auto' />;

  return (
    <div>
      <div className='mt-6'>
        <Label>To</Label>
        <InputField
          value={transferToAddress}
          onChange={(e) => setTransferToAddress(e.target.value)}
          placeholder='To address...'
          disabled={shouldDisable(transfer)}
        />
        <span className='flex items-center justify-between mt-3'>
          <label className='font-semibold uppercase text-xs'>Amount</label>
          {transferAmount !== undefined && (
            <h4 className='text-white text-sm'>
              {planckToDecimalFormatted(transferAmount, {
                api: erc20.contract.api,
                symbol: 'CLAMS',
              })}
            </h4>
          )}
        </span>
        <BigIntInputField
          value={transferAmount.toString()}
          onDigitChange={setTransferAmount}
          placeholder='Enter the amount'
          disabled={shouldDisable(transfer)}
        />
        <Button
          disabled={
            shouldDisable(transfer) || !transferToAddress || !transferAmount
          }
          onClick={() =>
            transfer.signAndSend([transferToAddress, transferAmount])
          }
          className='mt-3 w-full'
        >
          {isPendingSignature(transfer)
            ? 'Please sign transaction...'
            : shouldDisable(transfer)
            ? 'Transferring...'
            : 'Transfer'}
        </Button>
      </div>

      <div className='mt-6'>
        <Label>Spender</Label>
        <InputField
          value={approveSpender}
          onChange={(e) => setApproveSpender(e.target.value)}
          placeholder='Spender address...'
          disabled={shouldDisable(approve)}
        />
        <span className='flex items-center justify-between mt-3'>
          <label className='font-semibold uppercase text-xs'>Amount</label>
          {transferAmount !== undefined && (
            <h4 className='text-white text-sm'>
              {planckToDecimalFormatted(approveAmount, {
                api: erc20.contract.api,
                symbol: 'CLAMS',
              })}
            </h4>
          )}
        </span>
        <BigIntInputField
          value={approveAmount.toString()}
          onDigitChange={setApproveAmount}
          placeholder='Enter the amount'
          disabled={shouldDisable(approve)}
        />
        <Button
          disabled={shouldDisable(approve) || !approveSpender || !approveAmount}
          onClick={() => approve.signAndSend([approveSpender, approveAmount])}
          className='mt-3 w-full'
        >
          {isPendingSignature(approve)
            ? 'Please sign transaction...'
            : shouldDisable(approve)
            ? 'Approving...'
            : 'Approve'}
        </Button>
      </div>

      <div className='mt-6'>
        <Label>From</Label>
        <InputField
          value={transferFromAddress}
          onChange={(e) => setTransferFromAddress(e.target.value)}
          placeholder='From address...'
          disabled={shouldDisable(transferFrom)}
        />
        <Label>To</Label>
        <InputField
          value={transferFromToAddress}
          onChange={(e) => setTransferFromToAddress(e.target.value)}
          placeholder='To address...'
          disabled={shouldDisable(transferFrom)}
        />
        <span className='flex items-center justify-between mt-3'>
          <label className='font-semibold uppercase text-xs'>Amount</label>
          {transferAmount !== undefined && (
            <h4 className='text-white text-sm'>
              {planckToDecimalFormatted(transferFromAmount, {
                api: erc20.contract.api,
                symbol: 'CLAMS',
              })}
            </h4>
          )}
        </span>
        <BigIntInputField
          value={transferFromAmount.toString()}
          onDigitChange={setTransferFromAmount}
          placeholder='Enter the amount'
          disabled={shouldDisable(transferFrom)}
        />
        <Button
          disabled={
            shouldDisable(transferFrom) ||
            !transferFromAddress ||
            !transferFromToAddress ||
            !transferFromAmount
          }
          onClick={() =>
            transferFrom.signAndSend([
              transferFromAddress,
              transferFromToAddress,
              transferFromAmount,
            ])
          }
          className='mt-3 w-full'
        >
          {isPendingSignature(transferFrom)
            ? 'Please sign transaction...'
            : shouldDisable(transferFrom)
            ? 'Transferring...'
            : 'Transfer'}
        </Button>
      </div>
    </div>
  );
};
