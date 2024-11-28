import { DeployerProvider, InkLayout } from 'ui';
import metadata from '../assets/erc721.json';
import { Erc721 } from './components';

function App() {
  return (
    <InkLayout
      className='md:py-12 md:p-6 p-4 h-screen flex items-center justify-center'
      animationSrc='https://raw.githubusercontent.com/use-ink/ink-workshop/d819d10a35b2ac3d2bff4f77a96701a527b3ad3a/frontend/public/dark-sea-creatures.json'
    >
      <DeployerProvider
        {...{
          metadata,
          constructorArgs: {},
          constructorName: 'new',
          codeHash:
            '0xe27214c33419121ce913d12859f57807fcee75a5ca53c32e7c97dd5d48181507',
        }}
      >
        <Erc721 />
      </DeployerProvider>
    </InkLayout>
  );
}

export default App;
