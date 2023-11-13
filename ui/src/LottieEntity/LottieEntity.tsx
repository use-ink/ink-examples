import { Player } from '@lottiefiles/react-lottie-player';
import React from 'react';
import { ClassNameable } from '..';

type Props = {
  src: string;
} & ClassNameable;

export const LottieEntity: React.FC<Props> = ({ src, className }) => {
  return (
    <div className={className}>
      <Player autoplay loop src={src} />
    </div>
  );
};
