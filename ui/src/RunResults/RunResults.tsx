import classNames from 'classnames';
import React from 'react';
import { IApiProvider } from 'useink';
import { StorageDeposit, WeightV2 } from 'useink/core';
import { planckToDecimalFormatted } from 'useink/utils';

export interface RunResultsProps {
  title?: string;
  className?: string;
  contractAddress?: string;
  storageDeposit?: StorageDeposit;
  gasConsumed?: WeightV2;
  gasRequired?: WeightV2;
  chainApi: IApiProvider | undefined;
}

export const RunResults: React.FC<RunResultsProps> = ({
  title,
  className,
  storageDeposit,
  contractAddress,
  gasConsumed,
  gasRequired,
  chainApi,
}) => {
  return (
    <div className={classNames(className)}>
      {storageDeposit && (
        <div className='mt-6 w-full text-xs'>
          {title && <h2 className='uppercase font-semibold'>{title}</h2>}

          {contractAddress && (
            <hgroup>
              <h3 className='uppercase font-semibold'>Contract Address</h3>
              <h3>{contractAddress}</h3>
            </hgroup>
          )}

          {gasConsumed && (
            <div className='mt-3'>
              <h3 className='uppercase font-semibold'>Gas Consumed</h3>
              <ul className='p-0 list-none'>
                <li>refTime: {gasConsumed.refTime.toString()}</li>
                <li>proof size: {gasConsumed.proofSize.toString()}</li>
              </ul>
            </div>
          )}

          {gasRequired && (
            <div className='mt-3'>
              <h3 className='uppercase font-semibold'>Gas Required</h3>
              <ul className='p-0 list-none'>
                <li>refTime: {gasRequired.refTime.toString()}</li>
                <li>proof size: {gasRequired.proofSize.toString()}</li>
              </ul>
            </div>
          )}

          {storageDeposit && (
            <div className='mt-3'>
              <h3>
                Storage Deposit:{' '}
                {planckToDecimalFormatted(storageDeposit.asCharge, {
                  api: chainApi?.api,
                })}
              </h3>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
