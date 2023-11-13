import classNames from 'classnames';
import * as React from 'react';
import { Card, useUI } from '../..';
import screenMask from '../../assets/screen-mask.svg';
import { ManageWallet } from '../ManageWallet';

export const Screen: React.FC<React.PropsWithChildren> = ({ children }) => {
  const { view, showScreen, screenPosition } = useUI();

  return (
    <div
      className={classNames('absolute', showScreen && 'screenlines')}
      style={{
        maskImage: `url("${screenMask}")`,
        WebkitMaskImage: `url("${screenMask}")`,
        WebkitMaskRepeat: 'no-repeat',
        WebkitMaskSize: '100%',
        maskRepeat: 'no-repeat',
        maskSize: '100%',
        ...screenPosition,
      }}
    >
      <div className='py-32 h-full overflow-y-scroll'>
        {view === 'contract' && children}
        {view === 'wallet' && (
          <div className='flex flex-col items-center h-full justify-center w-full'>
            <Card className='mx-auto w-full'>
              <ManageWallet />
            </Card>
          </div>
        )}
      </div>
    </div>
  );
};
