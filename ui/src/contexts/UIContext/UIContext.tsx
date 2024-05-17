import { Howl } from 'howler';

import React, {
  PropsWithChildren,
  createContext,
  useCallback,
  useEffect,
  useMemo,
  useState,
} from 'react';

type View = 'off' | 'contract' | 'wallet';

interface ScreenPosition {
  top: number;
  left: number;
  right: number;
  bottom: number;
  width: number;
  height: number;
}

export interface UIState {
  showScreen: boolean;
  showClearContract: boolean;
  setShowClearContract: (show: boolean) => void;
  setView: (view: View) => void;
  view: View;
  playAudio: boolean;
  setPlayAudio: (play: boolean) => void;
  playLedSwitch: () => void;
  setScreenPosition: (pos: ScreenPosition) => void;
  screenPosition: ScreenPosition | undefined;
}

export const UIContext = createContext<UIState>({
  setView: () => null,
  view: 'contract',
  showScreen: true,
  setShowClearContract: () => null,
  showClearContract: true,
  setPlayAudio: () => null,
  playLedSwitch: () => null,
  playAudio: true,
  setScreenPosition: () => null,
  screenPosition: undefined,
});

export const UIProvider: React.FC<PropsWithChildren> = ({ children }) => {
  const [waterSounds, setWaterSounds] = useState<Howl>();
  const [ledEffect, setLedEffect] = useState<Howl>();
  const [showClearContract, setShowClearContract] = useState(false);
  const [playAudio, setPlayAudio] = useState(false);
  const [view, setView] = useState<View>('contract');
  const showScreen = useMemo(() => view !== 'off', [view]);
  const [screenPosition, setScreenPosition] = useState<ScreenPosition>();

  useEffect(() => {
    setLedEffect(
      new Howl({
        src: 'https://github.com/use-ink/ink-examples/raw/sr/submarine/ui/src/assets/audio/led.wav',
        volume: 0.8,
        html5: true,
      }),
    );

    setWaterSounds(
      new Howl({
        src: 'https://github.com/use-ink/ink-examples/raw/sr/submarine/ui/src/assets/audio/underwater.mp3',
        loop: true,
        volume: 1,
        html5: true,
      }).on('load', () => {
        setPlayAudio(true);
      }),
    );
  }, []);

  const playLedSwitch = useCallback(() => {
    playAudio && ledEffect?.play();
  }, [ledEffect, playAudio]);

  useEffect(() => {
    playAudio ? waterSounds?.play() : waterSounds?.stop();
  }, [playAudio]);

  return (
    <UIContext.Provider
      value={{
        view,
        setView,
        showScreen,
        screenPosition,
        setScreenPosition,
        showClearContract,
        setShowClearContract,
        playAudio,
        setPlayAudio,
        playLedSwitch,
      }}
    >
      {children}
    </UIContext.Provider>
  );
};
