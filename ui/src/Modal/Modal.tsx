import { Dialog, Transition } from '@headlessui/react';
import classNames from 'classnames';
import React, { Fragment, PropsWithChildren } from 'react';

type Props = {
  open: boolean;
  handleClose?: () => void;
  className?: string;
};

export const Modal: React.FC<PropsWithChildren<Props>> = ({
  open,
  handleClose,
  children,
  className,
}) => {
  const containerClasses = classNames(
    'inline-block bg-brand-800 border border-white/10 rounded-2xl overflow-scroll shadow-xl transform transition-all w-full max-w-3xl',
    className,
  );

  return (
    <Transition.Root show={open} as={Fragment}>
      <Dialog
        as='div'
        static
        className='fixed z-40 inset-0 overflow-y-auto'
        open={open}
        onClose={() => handleClose?.()}
      >
        <div className='flex flex-col items-center sm:py-8 justify-start h-screen lg:pt-32 pt-4 px-4 pb-20 text-center'>
          <Transition.Child
            as={Fragment}
            enter='ease-out duration-300'
            enterFrom='opacity-0'
            enterTo='opacity-100'
            leave='ease-in duration-200'
            leaveFrom='opacity-100'
            leaveTo='opacity-0'
          >
            <Dialog.Overlay className='fixed inset-0 bg-gray-900 bg-opacity-75 transition-opacity' />
          </Transition.Child>
          <Transition.Child
            as={Fragment}
            enter='ease-out duration-300'
            enterFrom='opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95'
            enterTo='opacity-100 translate-y-0 sm:scale-100'
            leave='ease-in duration-200'
            leaveFrom='opacity-100 translate-y-0 sm:scale-100'
            leaveTo='opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95'
          >
            <div className={containerClasses}>{children}</div>
          </Transition.Child>
        </div>
      </Dialog>
    </Transition.Root>
  );
};
