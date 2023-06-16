import { Button, ButtonProps, useUI } from '..';
import React from 'react';

export const ConnectButton: React.FC<Omit<ButtonProps, 'onClick'>> = ({
  className,
  children = 'Connect Wallet',
  ...rest
}) => {
  const { setShowConnectWallet } = useUI();

  return (
    <Button
      className={className}
      onClick={() => setShowConnectWallet(true)}
      {...rest}
    >
      {children}
    </Button>
  );
};
