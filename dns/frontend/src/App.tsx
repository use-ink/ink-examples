import {
  development,
  UseInkathonProvider,
  useInkathon,
  SubstrateDeployment,
  useRegisteredContract,
  contractQuery,
  decodeOutput,
  contractTx,
  useBalance,
} from "@scio-labs/use-inkathon";
import DNS_ABI from "./dns.json";
import { useRef, useState } from "react";
import { blake2AsHex } from "@polkadot/util-crypto";

const usePromise = <T,>(promise: () => Promise<T>) => {
  const [result, setResult] = useState<T | null>(null);
  const [error, setError] = useState<unknown | null>(null);
  const [loading, setLoading] = useState(false);

  const execute = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await promise();
      setResult(result);
    } catch (error) {
      setError(error);
    } finally {
      setLoading(false);
    }
  };

  return { result, error, loading, execute };
};

function App() {
  const { isConnected } = useInkathon();
  return (
    <div className="flex flex-col gap-4 w-3/4">
      <ConnectionState />
      {isConnected && (
        <>
          <RegisterName />
          <SetAddress />
          <GetAddress />
        </>
      )}
    </div>
  );
}

const ConnectionState = () => {
  const { connect, disconnect, activeChain, activeAccount } = useInkathon();
  const { contract } = useRegisteredContract("dns");
  const balance = useBalance(activeAccount?.address, true);

  if (!activeAccount) {
    return (
      <button onClick={() => (connect ? connect() : undefined)}>Connect</button>
    );
  }

  return (
    <div className="card">
      <div className="grid grid-cols-3 overflow-hidden">
        <div>
          <div className="text-sm text-slate-500">Chain</div>
          {activeChain && (
            <div className="text-lg font-semibold">{activeChain.name}</div>
          )}
        </div>
        <div>
          <div className="text-sm text-slate-500">Active Account</div>
          {activeAccount && (
            <div className="text-lg font-semibold">{activeAccount.name}</div>
          )}
          <div className="text-sm text-ellipsis overflow-hidden">
            {activeAccount.address}
          </div>
        </div>

        <div>
          <div className="text-sm text-slate-500">Account Balance</div>
          {balance && (
            <div className="text-lg font-semibold">
              {balance.balanceFormatted}
            </div>
          )}
        </div>
      </div>

      <div>
        <div className="text-sm">Contract</div>
        {contract && (
          <div className="text-lg font-semibold">{contract?.address}</div>
        )}
      </div>
      {activeAccount && <button onClick={disconnect}>Disconnect</button>}
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
        <h3 className="font-semibold text-lg">Lookup Name</h3>
        <p className="slate-500">
          Resolves <code>Address</code> for a given <code>Name</code>
        </p>
      </div>

      <form className="flex flex-row gap-1" action={"#"}>
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

const RegisterName = () => {
  const { api, activeAccount } = useInkathon();
  const [name, setName] = useState("");
  const [result, setResult] = useState<null | Awaited<
    ReturnType<typeof contractTx>
  >>(null);
  const [error, setError] = useState<null | string>(null);
  const { contract } = useRegisteredContract("dns");

  const register = async (name: string) => {
    if (!contract || !api || !activeAccount || !name) return;

    setError(null);

    try {
      const result = await contractTx(
        api,
        activeAccount.address,
        contract,
        "register",
        {},
        [blake2AsHex(name)]
      );
      console.log({ result });

      setResult(result);
    } catch (error: unknown) {
      setResult(null);
      if (
        typeof error === "object" &&
        error &&
        "errorMessage" in error &&
        typeof error.errorMessage === "string"
      )
        setError(error.errorMessage);
      else {
        setError("Unknown error");
      }
    }
  };

  return (
    <div className="card">
      <div>
        <h3 className="font-semibold text-lg">Register a New Name</h3>
        <p className="slate-500">
          Claim ownership of the given <code>name</code>
        </p>
      </div>

      <form className="flex flex-row gap-1" action="#">
        <input
          placeholder="Name to register"
          onChange={(event) => setName(event.target.value)}
          id="name"
        />
        <button
          onClick={() => {
            register(name);
          }}
        >
          Register
        </button>
      </form>
      <hr />
      {error && <div className="error">{error}</div>}
      {result && <textarea value={JSON.stringify(result, null, 2)}></textarea>}
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

  console.log({ result, error, loading });

  return (
    <div className="card">
      <div>
        <h3 className="font-semibold text-lg">Set AccountId For Name</h3>
        <p className="slate-500">
          Set the <code>AccountId</code> which the given <code>name</code>{" "}
          should resolve to
        </p>
      </div>

      <form className="flex flex-col gap-1" action="#">
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
        <button onClick={execute}>Set</button>
      </form>
      <hr />
      {/* {error && <div className="error">{JSON.stringify(error)}</div>} */}
      {result && <textarea value={JSON.stringify(result, null, 2)}></textarea>}
    </div>
  );
};

const getDeployments = async (): Promise<SubstrateDeployment[]> => {
  return [
    {
      contractId: "dns",
      networkId: "development",
      abi: DNS_ABI,
      address: "5FYQXAgTz4B28dAAb8wchiKjhM2MeWVshnYgwHCaoqvwtbCp",
    },
  ];
};

function WrappedApp() {
  return (
    <UseInkathonProvider
      appName="DNS Frontend Example"
      deployments={getDeployments()}
      defaultChain={development}
    >
      <App />
    </UseInkathonProvider>
  );
}

export default WrappedApp;
