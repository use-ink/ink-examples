import React from 'react';
import ReactDOM from 'react-dom/client';
import 'ui/style.css';
import { UseInkProvider } from 'useink';
import { RococoContractsTestnet } from 'useink/chains';
import { NotificationsProvider } from 'useink/notifications';
import App from './App.tsx';
import './Global.css';
import metadata from './assets/basic_contract_caller.json';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <UseInkProvider
      config={{
        dappName: metadata.contract.name,
        chains: [RococoContractsTestnet],
        caller: {
          default: '5EyR7vEk7DtvEWeefGcXXMV6hKwB8Ex5uvjHufm466mbjJkR',
        },
      }}
    >
      <NotificationsProvider>
        <App />
      </NotificationsProvider>
    </UseInkProvider>
  </React.StrictMode>,
);
