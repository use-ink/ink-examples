import classNames from 'classnames';
import React, {
  AnchorHTMLAttributes,
  DetailedHTMLProps,
  PropsWithChildren,
} from 'react';

type LinkHTMLProps = DetailedHTMLProps<
  AnchorHTMLAttributes<HTMLAnchorElement>,
  HTMLAnchorElement
>;

type LinkProps = PropsWithChildren<LinkHTMLProps>;

export const Link: React.FC<LinkProps> = ({ children, className, ...rest }) => {
  const classes = classNames(
    'underline text-sm text-brand-300 font-semibold transition duration-25 hover:text-brand-450',
    className,
  );

  return (
    <a className={classes} {...rest}>
      {children}
    </a>
  );
};
