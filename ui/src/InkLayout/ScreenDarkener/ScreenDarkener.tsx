import classNames from 'classnames';
import * as React from 'react';
import { useUI } from '../..';

export const ScreenDarkener: React.FC = () => {
  const { showScreen } = useUI();
  return (
    <div
      className={classNames(
        'absolute left-0 right-0 top-0 bottom-0 transition duration-75',
        showScreen ? 'bg-brand-1000/60' : 'bg-transparent',
      )}
    />
  );
};
