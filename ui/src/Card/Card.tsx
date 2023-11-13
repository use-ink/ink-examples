import classNames from 'classnames';
import React from 'react';
import { InkComponent } from '../types';

export const Card: React.FC<InkComponent> = ({ children, className }) => {
  const classes = classNames(
    'text-white/90 lg:max-w-[50%] md:max-w-[65%] max-w-[75%] rounded-2xl bg-white/10 p-3',
    className,
  );

  return (
    <div className={classNames('rotate-x-[50deg]', classes)}>{children}</div>
  );
};
