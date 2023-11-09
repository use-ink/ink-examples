import { InkComponent } from '../types';
import classNames from 'classnames';
import React from 'react';

export const Card: React.FC<InkComponent> = ({ children, className }) => {
  const classes = classNames(
    'bg-brand-500 text-white/90 rounded-xl max-w-3xl',
    className,
  );

  return <div className={classes}>{children}</div>;
};
