import classnames from 'classnames';
import React, {
  ChangeEventHandler,
  DetailedHTMLProps,
  InputHTMLAttributes,
} from 'react';

interface Props
  extends DetailedHTMLProps<
    InputHTMLAttributes<HTMLInputElement>,
    HTMLInputElement
  > {
  onChange: ChangeEventHandler<HTMLInputElement>;
  value: string;
  placeholder?: string;
  disabled?: boolean;
}

export const InputField: React.FC<Props> = ({ className, ...props }) => {
  return (
    <input
      className={classnames(
        'text-center grow focus:ring-0 focus:ring-offset-0 disabled:cursor-not-allowed rounded-md px-3 py-2 text-sm',
        'w-full placeholder:font-normal placeholder:text-sm placeholder:text-white/70 bg-white/20 text-white',
        'disabled:text-white/70 disabled:bg-white/40 transition duration-75 outline-none focus:ring-2 ring-white/50',
        className,
      )}
      type='text'
      {...props}
    />
  );
};
