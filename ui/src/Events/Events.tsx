import classNames from 'classnames';
import React from 'react';
import { EventRecord } from 'useink/core';
import {
  asContractInstantiatedEvent,
  formatEventName,
  isContractInstantiatedEvent,
  isExtrinsicFailedEvent,
} from 'useink/utils';
import { ClassNameable } from '..';

export interface Props extends ClassNameable {
  events?: EventRecord[];
}

export const Events: React.FC<Props> = ({ events, className }) => {
  if (!events || events.length === 0) return null;

  return (
    <ul className={classNames('text-xs', className)}>
      {events.map((event) => (
        <li
          key={event.toString()}
          className={classNames(
            'bg-brand-500 rounded-md px-1 py-1 mt-1 w-full',
            isContractInstantiatedEvent(event) && 'bg-success-500',
            isExtrinsicFailedEvent(event) && 'bg-warning-500',
          )}
        >
          {isContractInstantiatedEvent(event) ? (
            <div>
              <h4 className='font-bold'>{formatEventName(event)}</h4>
              <h4>Deployer: {asContractInstantiatedEvent(event)?.deployer}</h4>

              <h4>
                Contract Address:{' '}
                {asContractInstantiatedEvent(event)?.contractAddress}
              </h4>
            </div>
          ) : (
            formatEventName(event)
          )}
        </li>
      ))}
    </ul>
  );
};
