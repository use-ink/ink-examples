import { blake2AsHex } from "@polkadot/util-crypto";
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
import { useRef, useState } from "react";

import CONTRACT_METADATA from "./dns.json";
const CONTRACT_NAME = "dns";

const getDeployments = async (): Promise<SubstrateDeployment[]> => {
  return [
    {
      contractId: CONTRACT_NAME,
      networkId: rococo.network,
      abi: CONTRACT_METADATA,
      address: "5GWCUiApMhV3QYK4RedaLpbhcCBWLeGVT2wtZPfCHhnHxoud",
    },
  ];
};

export default function WrappedApp() {
  return (
    <UseInkathonProvider
      appName="Flipper Frontend Example"
      deployments={getDeployments()}
      defaultChain={rococo}
    >
      <App />
    </UseInkathonProvider>
  );
}

function App() {
  const { isConnected } = useInkathon();
  return (
    <div className="w-screen flex justify-center  p-4">
      <div className="max-w-2xl flex flex-col gap-4 ">
        <ConnectionState />
        {isConnected && (
          <>
            <RegisterName />
            <SetAddress />
            <GetAddress />
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

const RegisterName = () => {
  const { api, activeAccount } = useInkathon();

  const [name, setName] = useState("");
  const nameRef = useRef<HTMLInputElement>(null);

  const { contract } = useRegisteredContract("dns");

  const {
    loading,
    error,
    result,
    execute: register,
  } = usePromise(async () => {
    if (!contract || !api || !activeAccount || !nameRef.current) return;
    setName(nameRef.current.value);
    return contractTx(api, activeAccount.address, contract, "register", {}, [
      blake2AsHex(nameRef.current.value),
    ]);
  });

  return (
    <div className="card">
      <div>
        <h3 className="font-semibold text-lg">Step 1: Register a New Name</h3>
        <p className="slate-500">
          Claim ownership of the given <code>name</code>
        </p>
      </div>

      <form className="flex flex-row gap-1" action="#">
        <input placeholder="Name to register" ref={nameRef} />
        <button
          type="submit"
          disabled={loading}
          onClick={() => {
            register();
          }}
        >
          {loading ? "Registering..." : "Register"}
        </button>
      </form>

      {error && (
        <>
          <hr />
          <div className="error">{error}</div>
        </>
      )}

      {result && !!result.successEvent && (
        <>
          <hr />
          <div className="success">
            Name <code>{name}</code> registered!
          </div>
        </>
      )}
    </div>
  );
};

const SetAddress = () => {
  const { api, activeAccount } = useInkathon();
  const [name, setName] = useState("");
  const [accountId, setAccountId] = useState("");

  const { contract } = useRegisteredContract("dns");

  const { execute, loading, result, error } = usePromise(async () => {
    if (!contract || !api || !activeAccount || !name) return;

    return contractTx(api, activeAccount.address, contract, "setAddress", {}, [
      blake2AsHex(name),
      accountId,
    ]);
  });

  return (
    <div className="card">
      <div>
        <h3 id="SetAddress" className="font-semibold text-lg">
          Step 2: Set AccountId For Name
        </h3>
        <p className="slate-500">
          Set the <code>AccountId</code> which the given <code>name</code>{" "}
          should resolve to
        </p>
      </div>

      <form className="flex flex-col gap-1" action="#SetAddress">
        <input
          placeholder="Name"
          onChange={(event) => setName(event.target.value)}
          id="name"
        />
        <input
          placeholder="AccountId"
          onChange={(event) => setAccountId(event.target.value)}
          id="account_id"
        />
        <button type="submit" disabled={loading} onClick={execute}>
          {loading ? "Updating..." : "Set"}
        </button>
      </form>

      {error && (
        <>
          <hr />
          <div className="error">{error}</div>
        </>
      )}
      {result && !!result.successEvent && (
        <>
          <hr />
          <div className="success">
            <div>
              Name <code>{name}</code> now resolves to:
            </div>
            <div>
              <code>{accountId}</code>
            </div>
          </div>
        </>
      )}
    </div>
  );
};

const GetAddress = () => {
  const { api } = useInkathon();
  const inputRef = useRef<HTMLInputElement>(null);
  const [name, setName] = useState("");
  const [result, setResult] = useState<null | ReturnType<typeof decodeOutput>>(
    null
  );
  const { contract } = useRegisteredContract("dns");

  const getAddress = async (name: string) => {
    if (!contract || !api || !name) return;

    const result = await contractQuery(
      api,
      "",
      contract,
      "getAddress",
      undefined,
      [blake2AsHex(name)]
    );

    setResult(decodeOutput(result, contract, "getAddress"));
    setName(name);
  };

  return (
    <div className="card">
      <div>
        <h3 id="GetAddress" className="font-semibold text-lg">
          Step 3: Lookup Name
        </h3>
        <p className="slate-500">
          Resolves <code>Address</code> for a given <code>Name</code>
        </p>
      </div>

      <form className="flex flex-row gap-1" action={"#GetAddress"}>
        <input ref={inputRef} placeholder="Name to lookup" id="name" />
        <button
          type="submit"
          onClick={() => {
            if (inputRef.current) inputRef.current.blur();
            if (inputRef.current?.value) {
              getAddress(inputRef.current?.value);
            }
          }}
        >
          Lookup
        </button>
      </form>
      {result && (
        <>
          <hr />
          <div>
            <code className="font-bold">{name}</code> resolves to:
          </div>
          <div>{result.decodedOutput}</div>
        </>
      )}
    </div>
  );
};

const usePromise = <T,>(promise: () => Promise<T>) => {
  const [result, setResult] = useState<T | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const execute = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await promise();
      setResult(result);
    } catch (error) {
      if (
        typeof error === "object" &&
        error &&
        "errorMessage" in error &&
        typeof error.errorMessage === "string"
      )
        setError(error.errorMessage);
      else {
        console.error(error);
        setError("Unknown error, check console");
      }
    } finally {
      setLoading(false);
    }
  };

  return { result, error, loading, execute };
};
