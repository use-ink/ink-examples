import { PropsWithChildren } from 'react';

export interface ClassNameable {
  className?: string;
}

export type InkComponent = PropsWithChildren<ClassNameable>;
