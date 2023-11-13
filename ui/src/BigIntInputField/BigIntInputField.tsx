import React, { DetailedHTMLProps, InputHTMLAttributes } from 'react';
import { InputField } from '..';

interface Props
  extends DetailedHTMLProps<
    InputHTMLAttributes<HTMLInputElement>,
    HTMLInputElement
  > {
  value: string;
  onDigitChange: (digits: bigint) => void;
}

export const BigIntInputField: React.FC<Props> = ({
  className,
  onDigitChange,
  ...props
}) => {
  const handleChange = (val: string) => {
    if (/^\d*$/.test(val)) onDigitChange(BigInt(val));
  };

  return (
    <InputField
      className={className}
      {...props}
      onChange={(e) => handleChange(e.target.value)}
    />
  );
};
