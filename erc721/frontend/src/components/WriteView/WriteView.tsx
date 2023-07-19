import { useState } from 'react';
import {
  Button,
  ConnectButton,
  InputField,
  NumberInput,
  ToggleSwitch,
} from 'ui';
import { ChainContract, useTx, useWallet } from 'useink';
import { useTxNotifications } from 'useink/notifications';
import { isPendingSignature, shouldDisable } from 'useink/utils';

interface Props {
  erc721: ChainContract;
}

export const WriteView: React.FC<Props> = ({ erc721 }) => {
  const { account } = useWallet();

  const [mintTokenId, setMintTokenId] = useState(1);
  const mint = useTx<null>(erc721, 'mint');
  useTxNotifications(mint);

  const [burnTokenId, setBurnTokenId] = useState(1);
  const burn = useTx<null>(erc721, 'burn');
  useTxNotifications(burn);

  const [isApprovedForAll, setIsApprovedForAll] = useState(true);
  const [approvalForAllAccount, setApprovalForAllAccount] = useState('');
  const setApprovalForAll = useTx<null>(erc721, 'setApprovalForAll');
  useTxNotifications(setApprovalForAll);

  const [approveTokenId, setApproveTokenId] = useState(1);
  const [approvalForTokenAccount, setApprovalForTokenAccount] = useState('');
  const approve = useTx<null>(erc721, 'approve');
  useTxNotifications(approve);

  const [transferTokenId, setTransferTokenId] = useState(1);
  const [transferToAccount, setTransferToAccount] = useState('');
  const transfer = useTx<null>(erc721, 'transfer');
  useTxNotifications(transfer);

  const [transferFromTokenId, setTransferFromTokenId] = useState(1);
  const [transferFromOwnerAccount, setTransferFromOwnerAccount] = useState('');
  const [transferFromToAccount, setTransferFromToAccount] = useState('');
  const transferFrom = useTx<null>(erc721, 'transferFrom');
  useTxNotifications(transferFrom);

  if (!account) return <ConnectButton className='mt-6 mx-auto' />;

  return (
    <div>
      <div className='mt-6'>
        <label className='mt-6 font-semibold uppercase text-xs'>Token ID</label>
        <NumberInput
          value={mintTokenId}
          min={1}
          max={10}
          onChange={setMintTokenId}
          placeholder='Enter a token ID to mint...'
          disabled={shouldDisable(mint)}
        />
        <Button
          className='w-full mt-3'
          disabled={shouldDisable(mint) || !mintTokenId}
          onClick={() =>
            mint.signAndSend([mintTokenId], { defaultCaller: true })
          }
        >
          {isPendingSignature(mint)
            ? 'Sign transaction'
            : shouldDisable(mint)
            ? 'Minting...'
            : 'Mint'}
        </Button>
      </div>

      <div className='mt-6'>
        <label className='mt-6 font-semibold uppercase text-xs'>Token ID</label>
        <NumberInput
          value={burnTokenId}
          min={1}
          max={10}
          onChange={setBurnTokenId}
          disabled={shouldDisable(burn)}
        />
        <Button
          className='w-full mt-3'
          disabled={shouldDisable(burn)}
          onClick={() => burn.signAndSend([burnTokenId])}
        >
          {isPendingSignature(burn)
            ? 'Sign transaction'
            : shouldDisable(burn)
            ? 'Burning...'
            : 'Burn'}
        </Button>
      </div>

      <div className='mt-6'>
        <label className='mt-6 font-semibold uppercase text-xs'>Approve</label>
        <ToggleSwitch
          enabled={isApprovedForAll}
          handleClick={() => setIsApprovedForAll(!isApprovedForAll)}
        />

        <label className='mt-6 font-semibold uppercase text-xs'>Operator</label>
        <InputField
          value={approvalForAllAccount}
          onChange={(e) => setApprovalForAllAccount(e.target.value)}
          placeholder='Enter an Address...'
          disabled={shouldDisable(setApprovalForAll)}
        />
        <Button
          className='w-full mt-3'
          disabled={shouldDisable(setApprovalForAll) || !approvalForAllAccount}
          onClick={() =>
            setApprovalForAll.signAndSend([
              approvalForAllAccount,
              isApprovedForAll,
            ])
          }
        >
          {isPendingSignature(setApprovalForAll)
            ? 'Sign transaction'
            : shouldDisable(setApprovalForAll)
            ? 'Setting approval...'
            : 'Set Approval for All'}
        </Button>
      </div>

      <div className='mt-6'>
        <label className='mt-6 font-semibold uppercase text-xs'>Token ID</label>
        <NumberInput
          value={approveTokenId}
          min={1}
          max={10}
          onChange={setApproveTokenId}
          placeholder='Enter a token ID to approve...'
          disabled={shouldDisable(approve)}
        />

        <label className='mt-6 font-semibold uppercase text-xs'>Operator</label>
        <InputField
          value={approvalForTokenAccount}
          onChange={(e) => setApprovalForTokenAccount(e.target.value)}
          placeholder='Enter an Address...'
          disabled={shouldDisable(approve)}
        />
        <Button
          className='w-full mt-3'
          disabled={shouldDisable(approve) || !approvalForTokenAccount}
          onClick={() =>
            approve.signAndSend([approvalForTokenAccount, approveTokenId])
          }
        >
          {isPendingSignature(approve)
            ? 'Sign transaction'
            : shouldDisable(approve)
            ? 'Approving...'
            : 'Approve'}
        </Button>
      </div>

      <div className='mt-6'>
        <label className='mt-6 font-semibold uppercase text-xs'>Token ID</label>
        <NumberInput
          value={transferTokenId}
          min={1}
          max={10}
          onChange={setTransferTokenId}
          disabled={shouldDisable(transfer)}
        />

        <label className='mt-6 font-semibold uppercase text-xs'>
          New Owner
        </label>
        <InputField
          value={transferToAccount}
          onChange={(e) => setTransferToAccount(e.target.value)}
          placeholder='Enter an Address...'
          disabled={shouldDisable(transfer)}
        />
        <Button
          className='w-full mt-3'
          disabled={shouldDisable(transfer) || !transferToAccount}
          onClick={() =>
            transfer.signAndSend([transferToAccount, transferTokenId])
          }
        >
          {isPendingSignature(transfer)
            ? 'Sign transaction'
            : shouldDisable(transfer)
            ? 'Transferring...'
            : 'Transfer'}
        </Button>
      </div>

      <div className='mt-6'>
        <label className='mt-6 font-semibold uppercase text-xs'>Token ID</label>
        <NumberInput
          value={transferFromTokenId}
          min={1}
          max={10}
          onChange={setTransferFromTokenId}
          disabled={shouldDisable(transferFrom)}
        />

        <label className='mt-6 font-semibold uppercase text-xs'>
          Current Owner
        </label>
        <InputField
          value={transferFromOwnerAccount}
          onChange={(e) => setTransferFromOwnerAccount(e.target.value)}
          placeholder='Enter an Address...'
          disabled={shouldDisable(transferFrom)}
        />

        <label className='mt-6 font-semibold uppercase text-xs'>
          New Owner
        </label>
        <InputField
          value={transferFromToAccount}
          onChange={(e) => setTransferFromToAccount(e.target.value)}
          placeholder='Enter an Address...'
          disabled={shouldDisable(transferFrom)}
        />
        <Button
          className='w-full mt-3'
          disabled={
            shouldDisable(transferFrom) ||
            !transferFromOwnerAccount ||
            !transferFromToAccount
          }
          onClick={() =>
            transferFrom.signAndSend([
              transferFromOwnerAccount,
              transferFromToAccount,
              transferFromTokenId,
            ])
          }
        >
          {isPendingSignature(transferFrom)
            ? 'Sign transaction'
            : shouldDisable(transferFrom)
            ? 'Transferring...'
            : 'Transfer From'}
        </Button>
      </div>
    </div>
  );
};
