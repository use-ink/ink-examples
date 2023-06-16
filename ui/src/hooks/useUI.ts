import { UIContext, UIState } from '..';
import { useContext } from 'react';

export const useUI = (): UIState => useContext(UIContext);
