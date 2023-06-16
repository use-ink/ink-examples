import { Transition } from '@headlessui/react';
import classNames from 'classnames';
import React, { Fragment } from 'react';

type NotificationLevel = 'error' | 'info' | 'success' | 'warning';

type Props = {
  className?: string;
  message: string;
  show: boolean;
  type: NotificationLevel;
};

const BG_COLORS: Record<NotificationLevel, string> = {
  error: 'bg-error-500',
  info: 'bg-info-500',
  success: 'bg-success-500',
  warning: 'bg-warning-500',
};

export const Snackbar: React.FC<Props> = ({ show, message, type }) => (
  <Transition
    show={show}
    as={Fragment}
    enter='transform ease-out duration-300 transition'
    enterFrom='translate-y-2 opacity-0 sm:translate-y-0 sm:translate-x-2'
    enterTo='translate-y-0 opacity-100 sm:translate-x-0'
    leave='transition ease-in duration-100'
    leaveFrom='opacity-100'
    leaveTo='opacity-0'
  >
    <div className='flex items-end justify-end mt-1 drop-shadow-md'>
      <div className={classNames('rounded-lg px-4 py-2', BG_COLORS[type])}>
        <span
          className={classNames('text-sm font-medium text-white text-right')}
        >
          {message}
        </span>
      </div>
    </div>
  </Transition>
);
