# Concordium DApp starter
<!-- After fetching you can remove from here: -->

Starter project template for build DApps for the [Concordium blockchain](https://concordium.com).

**This template includes:**

- Smart contract setup with:
  - Simple counter smart contract.
  - Deploy script for smart contract deployment.
  - Setup for integration tests.
  - VSCode tasks for running build.
  - GitHub Workflow, checking formatting, linter warnings and running tests.
- TypeScript dApp setup with:
  - Basic setup using [Vite build tool](https://vitejs.dev/) and [React](https://react.dev/).
  - [`@concordium/ccd-js-gen`](https://www.npmjs.com/package/@concordium/ccd-js-gen) to generate TypeScript smart contract clients directly from the smart contract.
  - [Prettier](https://prettier.io/) and [ESLint](https://eslint.org/) recommeded setup.
  - GitHub Workflow, checking formatting, linter warnings and running tests.

_Fetch this repository follow the setup instructions below._

<!-- To here -->

## Setup

Make sure to have the following installed:

- [NodeJs](https://nodejs.org).
- Rust and cargo (Recommended to install using [rustup](https://rustup.rs)).
- Recent version of [cargo concordium](https://crates.io/crates/cargo-concordium) (Install using `cargo install --locked cargo-concordium` or use it through the Concordium VS-Code extension).

## Smart contracts

### Build

To build the smart contract, navigate to the `contracts/my-contract` directory and run:

```bash
cargo concordium build --out ./concordium-out/module.wasm.v1 --schema-embed
```

_The `--out ./concordium-out/module.wasm.v1` is important, since the frontend assumes this is the location of the built smart contract._

### Run tests

To run the tests for the smart contract, navigate to the `contracts/my-contract` directory and run:

```bash
cargo concordium test --out ./concordium-out/module.wasm.v1 --schema-embed
```

_This will also build the contract, the `--out ./concordium-out/module.wasm.v1` is important, since the frontend assumes this is the location of the built smart contract._

### Contract deploy-script

Scripts for deploying and setting up the smart contract can be found in `contract/my-contract`. [See the documentation here](./contracts/my-contract/deploy-scripts/README.md).

## Frontend

To setup and install dependencies for the frontend navigate to the `frontend` directory and run:

```bash
npm install
```

### Generate smart contract clients

This project is setup to generate TypeScript smart contract clients, directly from the smart contract module and the embedded schema. Make sure to build the smart contract modules as descriped above.

To generate the smart contract clients for the frontend navigate to the `frontend` directory and run:

```bash
npm run generate
```

### Development

To start a development environment make sure to first generate the smart contract clients, then run the following from the `frontend` directory:

```bash
npm run dev
```

This will launch a development server with hot module replacement enabled.

### Build

To start build the frontend make sure to first generate the smart contract clients, then run the following from the `frontend` directory:

```bash
npm run build
```

This will bundle the project into `frontend/dist` directory.

### Format code

To check for formatting issues run the following command from the `frontend` directory:

```bash
npm run format-check
```

To automatically fix formatting issues run the following command from the `frontend` directory:

```bash
npm run format-fix
```

### Lint code

To check for linting issues run the following command from the `frontend` directory:

```bash
npm run lint-check
```

To automatically fix linting issues run the following command from the `frontend` directory:

```bash
npm run lint-fix
```