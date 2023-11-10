import classNames from 'classnames';
import React, { PropsWithChildren } from 'react';

interface Props {
  className?: string;
}

export const Tabs: React.FC<PropsWithChildren<Props>> = ({
  children,
  className,
}) => (
  <div
    className={classNames(
      'flex md:flex-row items-center justify-stretch',
      className,
    )}
  >
    {children}
  </div>
);

interface TabProps {
  className?: string;
  isSelected: boolean;
  onClick: () => void;
}

export const Tab: React.FC<PropsWithChildren<TabProps>> = ({
  children,
  className,
  isSelected,
  onClick,
}) => (
  <button
    onClick={onClick}
    type='button'
    className={classNames(
      'md:first:rounded-l-full md:last:rounded-r-full px-2 py-1  hover:bg-white/20 flex-grow text-sm uppercase',
      isSelected
        ? 'hover:cursor-auto hover:bg-white/40 bg-white/40'
        : 'bg-white/10',
      className,
    )}
  >
    {children}
  </button>
);
