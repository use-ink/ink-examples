import { Switch } from '@headlessui/react';
import classNames from 'classnames';
import React from 'react';

interface Props {
  enabled: boolean;
  handleClick: () => void;
  screenReader?: string;
}

export const ToggleSwitch: React.FC<Props> = ({
  enabled,
  handleClick: handleClose,
  screenReader,
}) => (
  <div>
    <Switch
      checked={enabled}
      onChange={handleClose}
      className={classNames(
        'bg-white/30 relative inline-flex items-center flex-shrink-0 z-0',
        'h-[18px] w-[32px] border-transparent rounded-full cursor-pointer transition-colors',
        'ease-in-out duration-200 focus:outline-none focus-visible:ring-2',
        'focus-visible:ring-white focus-visible:ring-opacity-75',
      )}
    >
      {screenReader && <span className='sr-only'>{screenReader}</span>}
      <span
        aria-hidden='true'
        className={classNames(
          enabled
            ? 'translate-x-[13px] bg-white'
            : 'translate-x-[1px] bg-white/60',
          'pointer-events-none inline-block h-[16px] w-[18px]',
          'rounded-full shadow-lg transform ring-0 transition ease-in-out duration-200',
        )}
      />
    </Switch>
  </div>
);
