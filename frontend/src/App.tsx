import * as MyContract from "../generated/module_my_contract";
import * as SDK from "@concordium/web-sdk";

export function App() {
  return (
    <div className="centered column flex-1">
      <h1>Vite + React + Concordium</h1>
      <p className="pt-2">
        Edit <code>frontend/src/components/App.tsx</code> and save to test HMR
      </p>
      <p>
        Hello from the generated contract client:{" "}
        <code>{SDK.ContractName.toString(MyContract.contractName)}</code>.
      </p>
    </div>
  );
}
