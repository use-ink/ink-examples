import classNames from 'classnames';
import React, {
  ButtonHTMLAttributes,
  DetailedHTMLProps,
  PropsWithChildren,
} from 'react';

type ButtomHTMLProps = DetailedHTMLProps<
  ButtonHTMLAttributes<HTMLButtonElement>,
  HTMLButtonElement
>;

export type ButtonProps = PropsWithChildren<ButtomHTMLProps>;

export const Button: React.FC<ButtonProps> = ({
  children,
  className,
  onClick,
  ...rest
}) => {
  const classes = classNames(
    'bg-warning-500/90 hover:bg-warning-600/90 transition ease-in-out px-6 py-2 border-none block',
    'text-base tracking-wide font-semibold rounded-full disabled:bg-warning-500/60',
    'focus:ring-4 border-warning-600 ring-warning-600 disabled:cursor-not-allowed focus:outline-none focus:ring-offset-0 text-white',
    className,
  );

  return (
    <button type='button' className={classes} onClick={onClick} {...rest}>
      {children}
    </button>
  );
};
