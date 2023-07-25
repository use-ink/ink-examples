import React from 'react';
import { Button, ButtonProps, useUI } from '..';

export const ConnectButton: React.FC<Omit<ButtonProps, 'onClick'>> = ({
  className,
  children = 'Connect Wallet',
  ...rest
}) => {
  const { setView } = useUI();

  return (
    <Button className={className} onClick={() => setView('wallet')} {...rest}>
      {children}
    </Button>
  );
};
