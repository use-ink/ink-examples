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
    'bg-brand-900 hover:opacity-80 transition ease-in-out px-6 py-2 border-none disabled:text-opacity-70',
    'text-base tracking-wide font-semibold rounded-full disabled:opacity-50 disabled:hover:bg-opacity-50',
    'focus:ring-none disabled:cursor-not-allowed focus:outline-none focus:ring-0 focus:ring-offset-0 text-white',
    className,
  );

  return (
    <button type='button' className={classes} onClick={onClick} {...rest}>
      {children}
    </button>
  );
};
