import classNames from 'classnames';
import React from 'react';
import { InkComponent, Notifications } from '..';
import { LottieEntity } from '../LottieEntity';
import { UIProvider } from '../contexts';
import { Screen } from './Screen';
import { ScreenDarkener } from './ScreenDarkener';
import { Submarine } from './Submarine';

type Props = InkComponent & {
  animationSrc?: string;
};

const InkLayoutInner: React.FC<Props> = ({ children, animationSrc }) => {
  return (
    <div
      className={classNames(
        animationSrc && 'fixed top-0 right-0 left-0 bottom-0 w-full h-screen',
      )}
    >
      {animationSrc && (
        <LottieEntity
          src={animationSrc}
          className='absolute right-[0] left-[0] bottom-[0] water-effect'
        />
      )}

      <ScreenDarkener />
      <Submarine />
      <Screen>{children}</Screen>
      <Notifications />
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
