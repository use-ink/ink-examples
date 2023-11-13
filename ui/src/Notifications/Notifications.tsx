import React from 'react';
// eslint-disable-next-line import/no-unresolved
import { toNotificationLevel, useNotifications } from 'useink/notifications';
import { Snackbar } from '../Snackbar';

export const Notifications: React.FC = () => {
  const { notifications } = useNotifications();

  if (!notifications.length) return null;

  return (
    <ul className='fixed right-4 bottom-4 z-10 p-0 m-0'>
      {notifications.map((n) => (
        <li key={n.id} className='p-0 m-0 mb-2 max-w-[400]'>
          <Snackbar
            message={n.message}
            type={toNotificationLevel(n.type)}
            show
          />
        </li>
      ))}
    </ul>
  );
};
