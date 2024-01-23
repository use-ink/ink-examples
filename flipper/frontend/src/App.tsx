import {
  SubstrateDeployment,
  UseInkathonProvider,
  contractQuery,
  contractTx,
  decodeOutput,
  rococo,
  useBalance,
  useInkathon,
  useRegisteredContract,
} from "@scio-labs/use-inkathon";
import {
  QueryClient,
  QueryClientProvider,
  useMutation,
  useQuery,
} from "@tanstack/react-query";

import CONTRACT_METADATA from "./flipper.json";
const CONTRACT_NAME = "flipper";
const queryClient = new QueryClient();

const getDeployments = async (): Promise<SubstrateDeployment[]> => {
  return [
    {
      contractId: CONTRACT_NAME,
      networkId: rococo.network,
      abi: CONTRACT_METADATA,
      address: "5Fsk6oqWHJzMAQmkBTVzxxqZPPngLbHG48Tro3i53LC3quao",
    },
  ];
};

export default function WrappedApp() {
  return (
    <QueryClientProvider client={queryClient}>
      <UseInkathonProvider
        appName="Flipper Frontend Example"
        deployments={getDeployments()}
        defaultChain={rococo}
      >
        <App />
      </UseInkathonProvider>
    </QueryClientProvider>
  );
}

function App() {
  const { isConnected } = useInkathon();
  return (
    <div className="w-screen h-screen flex justify-center  p-4">
      <div className="max-w-2xl flex flex-col gap-4 ">
        <ConnectionState />
        {isConnected && (
          <>
            <FlipperInteraction />
          </>
        )}
      </div>
    </div>
  );
}

const ConnectionState = () => {
  const {
    connect,
    disconnect,
    isConnected,
    activeChain,
    activeAccount,
    setActiveAccount,
    accounts,
  } = useInkathon();
  const { contract } = useRegisteredContract(CONTRACT_NAME);
  const balance = useBalance(activeAccount?.address, true);

  if (!isConnected) {
    return (
      <div>
        <button type="button" onClick={() => (connect ? connect() : undefined)}>
          Connect
        </button>
      </div>
    );
  }

  return (
    <div className="card">
      <div className="grid grid-rows gap-2 overflow-hidden">
        {activeChain && (
          <div>
            <div className="text-sm text-slate-500">Chain</div>
            <div className="text-lg font-semibold">{activeChain.name}</div>
          </div>
        )}

        {activeAccount && accounts && (
          <div>
            <div className="text-sm text-slate-500">Active Account</div>

            <select
              value={activeAccount.address}
              onChange={(v) => {
                const selectedAccount = accounts.find(
                  (account) => account.address === v.target.value
                );

                if (selectedAccount && setActiveAccount)
                  setActiveAccount(selectedAccount);
              }}
            >
              {accounts.map((account) => (
                <option value={account.address} key={account.address}>
                  {account.name ? account.name : account.address}
                </option>
              ))}
            </select>
            <div className="text-sm text-ellipsis overflow-hidden">
              {activeAccount.address}
            </div>
          </div>
        )}

        {balance && (
          <div>
            <div className="text-sm text-slate-500">Account Balance</div>
            <div className="text-lg font-semibold">
              {balance.balanceFormatted}
            </div>
            <div className="text-sm text-ellipsis overflow-hidden">
              <a href="https://use.ink/5.x/faucet">
                Get Tokens from Testnet Faucet
              </a>
            </div>
          </div>
        )}

        {contract && (
          <div>
            <div className="text-sm text-slate-500">Contract</div>
            <div className="text-lg font-semibold text-ellipsis overflow-hidden">
              {contract?.address.toHex()}
            </div>
          </div>
        )}
      </div>
      <button type="button" onClick={disconnect}>
        Disconnect
      </button>
    </div>
  );
};

const FlipperInteraction = () => {
  const { api, activeAccount } = useInkathon();
  const { contract } = useRegisteredContract(CONTRACT_NAME);

  const { data: flipState, refetch: refetchFlipState } = useQuery({
    queryKey: ["flipper", "get"],
    queryFn: async () => {
      if (!api || !contract) throw Error("api or contract not available");
      const outcome = await contractQuery(api, "", contract, "get", {}, []);
      return decodeOutput(outcome, contract, "get");
    },
    enabled: !!api && !!contract,
  });

  const {
    mutateAsync: flip,
    isPending,
    error,
    data: flipResult,
  } = useMutation({
    mutationKey: ["flipper", "flip"],
    mutationFn: async () => {
      if (!contract) throw new Error("Contract not available");
      if (!api) throw new Error("API not available");
      if (!activeAccount) throw new Error("Account not available");

      return contractTx(api, activeAccount.address, contract, "flip", {}, []);
    },
    onSuccess: () => {
      refetchFlipState();
    },
  });

  return (
    <div className="card">
      <div>
        <h3 className="font-semibold text-lg">Flip </h3>
        <p className="slate-500">Change contracts storage value</p>
      </div>

      <div className="flex flex-row justify-between items-center">
        {flipState?.decodedOutput && (
          <div className="flex flow-row gap-2">
            <code>Flipper.get()</code>
            <div className="font-bold">{flipState?.decodedOutput}</div>
          </div>
        )}

        <button type="submit" disabled={isPending} onClick={() => flip()}>
          {isPending ? "flipping..." : "Flipper.flip()"}
        </button>
      </div>

      {error && (
        <>
          <hr />
          <div className="error">{JSON.stringify(error)}</div>
        </>
      )}

      {flipResult && !!flipResult.successEvent && (
        <>
          <hr />
          <div className="success">Value Flipped!</div>
        </>
      )}
    </div>
  );
};
