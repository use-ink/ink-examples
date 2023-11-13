import { useContext } from 'react';
import { UIContext, UIState } from '..';

export const useUI = (): UIState => useContext(UIContext);
