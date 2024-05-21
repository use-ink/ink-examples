import { DeployerProvider, InkLayout } from 'ui';
import metadata from '../assets/erc20.json';
import { Erc20 } from './components';

const ONE_BILLION_TOKENS = '1000000000000000000000';

function App() {
  return (
    <InkLayout
      className='md:py-12 md:p-6 p-4 h-screen flex items-center justify-center'
      animationSrc='https://raw.githubusercontent.com/use-ink/ink-workshop/d819d10a35b2ac3d2bff4f77a96701a527b3ad3a/frontend/public/dark-sea-creatures.json'
    >
      <DeployerProvider
        {...{
          metadata,
          constructorArgs: { totalSupply: ONE_BILLION_TOKENS },
          constructorName: 'new',
          codeHash:
            '0x2f88d986a619d7a1b81a3f6f886977fb3d56a2455b0368142fd4bf2e9d76d39e',
        }}
      >
        <Erc20 />
      </DeployerProvider>
    </InkLayout>
  );
}

export default App;
