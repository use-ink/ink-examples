import { Listbox, Transition } from '@headlessui/react';
import { CheckIcon, ChevronUpDownIcon } from '@heroicons/react/24/solid';
import classNames from 'classnames';
import React, { Fragment } from 'react';
import { useWallet } from 'useink';
import { ClassNameable } from '../types';

export const Accounts: React.FC<ClassNameable> = ({ className }) => {
  const { setAccount, account, accounts } = useWallet();

  if (!accounts?.length) return null;

  return (
    <div className={classNames('w-full', className)}>
      <h2 className='text-2xl font-bold mb-6 text-white'>Accounts</h2>

      <Listbox
        value={account}
        onChange={(a) => {
          setAccount(a);
        }}
      >
        <div className='relative'>
          <Listbox.Button
            className={classNames(
              'relative min-w-[200px] w-full cursor-default rounded-lg bg-violet-900 py-2 pl-3 pr-10 text-left',
              'focus:outline-none focus-visible:border-indigo-500 focus-visible:ring-2 focus-visible:ring-white',
              'focus-visible:ring-opacity-75 focus-visible:ring-offset-2 focus-visible:ring-offset-orange-300',
              'sm:text-sm hover:cursor-pointer hover:opacity-80',
            )}
          >
            <span className='block truncate text-white/70'>
              {account?.name || account?.address}
            </span>
            <span className='pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2'>
              <ChevronUpDownIcon
                className='h-5 w-5 text-gray-400'
                aria-hidden='true'
              />
            </span>
          </Listbox.Button>
          <Transition
            as={Fragment}
            leave='transition ease-in duration-100'
            leaveFrom='opacity-100'
            leaveTo='opacity-0'
          >
            <Listbox.Options
              className={classNames(
                'absolute mt-1 max-h-60 w-full overflow-auto rounded-md bg-violet-900 py-1 text-base shadow-lg ring-1',
                'ring-black ring-opacity-5 focus:outline-none sm:text-sm',
              )}
            >
              {accounts?.map((acc) => (
                <Listbox.Option
                  key={acc.address}
                  className={({ active }) =>
                    classNames(
                      'relative cursor-default select-none py-2 pl-10 pr-4 hover:cursor-pointer',
                      active ? 'bg-violet-800 text-gray-300' : 'text-gray-300',
                    )
                  }
                  value={acc}
                >
                  {() => {
                    const selected =
                      account && account.address === acc?.address;
                    return (
                      <>
                        <span
                          className={`block truncate ${
                            selected ? 'font-medium' : 'font-normal'
                          }`}
                        >
                          {acc.name || acc.address}
                        </span>

                        {selected && (
                          <span className='absolute inset-y-0 left-0 flex items-center pl-3 text-amber-600'>
                            <CheckIcon className='h-5 w-5' aria-hidden='true' />
                          </span>
                        )}
                      </>
                    );
                  }}
                </Listbox.Option>
              ))}
            </Listbox.Options>
          </Transition>
        </div>
      </Listbox>
    </div>
  );
};
