import classNames from 'classnames';
import React, { PropsWithChildren } from 'react';

interface Props {
  className?: string;
}

export const Label: React.FC<PropsWithChildren<Props>> = ({
  children,
  className,
}) => (
  <label className={classNames('font-semibold uppercase text-xs', className)}>
    {children}
  </label>
);
