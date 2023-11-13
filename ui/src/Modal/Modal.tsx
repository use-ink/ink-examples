import classNames from 'classnames';
import React, { PropsWithChildren } from 'react';
import { Card } from '../Card';

type Props = {
  open: boolean;
  handleClose?: () => void;
  className?: string;
};

export const Modal: React.FC<PropsWithChildren<Props>> = ({
  open,
  children,
  className,
}) => {
  if (!open) return null;

  return (
    <div className='absolute top-0 bottom-0 left-0 right-0'>
      <div className='bg-background-900/60 flex flex-col items-center justify-center h-full'>
        <Card
          className={classNames(
            '!bg-brand-500/95 shadow-sm relative max-w-md w-full',
            className,
          )}
        >
          {children}
        </Card>
      </div>
    </div>
  );
};
