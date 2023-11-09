import { InkComponent, Notifications, useUI } from '..';
import { ConnectWallet } from '../ConnectWallet';
import { LottieEntity } from '../LottieEntity';
import { Navbar } from '../Navbar';
import { UIProvider } from '../contexts';
import classNames from 'classnames';
import React from 'react';

type Props = InkComponent & {
  animationSrc?: string;
  withNotifications?: boolean;
  withNavbar?: boolean;
};

const InkLayoutInner: React.FC<Props> = ({
  children,
  animationSrc,
  className,
  withNotifications = true,
  withNavbar = true,
}) => {
  const { showConnectWallet, setShowConnectWallet } = useUI();

  return (
    <div
      className={classNames(
        animationSrc && 'fixed top-0 right-0 left-0 bottom-0 w-full h-screen',
      )}
    >
      {animationSrc && (
        <LottieEntity
          src={animationSrc}
          className='absolute right-[0] left-[0] bottom-[0]'
        />
      )}
      {withNavbar && <Navbar />}
      {withNotifications && <Notifications />}
      <ConnectWallet
        show={showConnectWallet}
        onClose={() => setShowConnectWallet(false)}
      />

      <div className={className}>{children}</div>
    </div>
  );
};

export const InkLayout: React.FC<Props> = (props) => {
  return (
    <UIProvider>
      <InkLayoutInner {...props} />
    </UIProvider>
  );
};
