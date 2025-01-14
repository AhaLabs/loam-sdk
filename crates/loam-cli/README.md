# loam-cli

Build smart contracts authored with Loam SDK, manage smart contract dependencies from a frontend, initialize new loam projects.

Loam CLI comes with three main commands:

* `loam init` - Generates a [Loam frontend](https://github.com/loambuild/template?tab=readme-ov-file) that includes an `environments.toml` file describing the network settings, accounts, and contracts for each environment your team builds against.
* `loam build` - Two build processes in one:
  * Build smart contracts. Essentially, this is a wrapper around `soroban build` that can be used to build any Soroban project's contracts. Like `soroban build`, this will build contracts using suggested settings, meaning that it functions as a shorthand for something like `cargo build --target wasm32-unknown-unknown`. But on top of that, `loam build` will also find all Loam dependencies, resolve interdependencies, and build them all in the correct order.
  * Build frontend clients. If the project contains an `environments.toml` file, `loam build` will match the environment specified by the `LOAM_ENV` environment variable (for `loam build`, the default is `production`) to a predictable starting state. It will turn the contracts you depend on (contract dependencies) into frontend packages (NPM dependencies), getting your frontend app to the point where it is ready to build or run with its own dev server. This is done in as low-intrusive a way as possible (for example, if contracts are already deployed, are they using the correct Wasm hash? Do they need to have their TTL extended? It will update these things, rather than re-deploy every time.)
* `loam dev` - Monitors `contracts/*` and `environments.toml` for changes and re-runs `loam build` as needed. It also defaults to `LOAM_ENV=development`, rather than `production`.

## Getting started with `loam init`
1. Install loam cli. We recommend using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) to install `cargo binstall loam-cli`
1. To create a loam project, run `loam init <PROJECT_PATH>`. This will create a [Loam frontend](https://github.com/loambuild/template?tab=readme-ov-file) project at the provided `<PROJECT_PATH>`.
1. In your project directory run the following commands to get up and running with a loam project with two example contracts:
* `cp .env.example .env`
* `npm install`
* `npm run dev`


## `loam dev` and `loam build` in Depth

`loam dev` and `loam build` essentially work the same, except that one is for testing and another for production.

### `loam  build`

`loam dev` is the contract-dependencies-to-NPM-dependencies toolchain. It turns the contracts you depend on (contract dependencies) into frontend packages (NPM dependencies), getting your app to the point where it is ready to build or run with its own dev server, such as `astro dev`. (This template uses Astro, but `loam-cli` itself is agnostic to how you run your JavaScript frontend. It would work equally well with `next dev`, or with Svelte or Vue or any other JavaScript frontend tool.)

Here's a full list of everything `loam build` will do:

1. Default to `production` environment. This environment setting can be changed with either the `--env` flag or with the `LOAM_ENV` environment variable.

2. Inspect the `environments.toml` file and get things to the specified predictable starting state:


   ```mermaid
   flowchart TD
     A[loam dev] -->|network| B(run-locally?)
     B -->|yes| C[start]
     B -->|no| D[check]
     A -->|accounts| E(mainnet?)
     E -->|yes| F[check]
     E -->|no| G[create & fund]
     A -->|contracts| H(local?)
     H -->|yes| I(workspace = true?)
     I -->|yes| J[build, deploy, init]
     I -->|no| K[spoon]
     H -->|no| L[check]
     J --> M[bind & import]
     K --> M
     L --> M
   ```

   - connect to the specified network, or run it with `soroban network start`
   - create and/or fund accounts
     → on mainnet, will instead check that accounts exist and are funded
   - For specified contracts:
     - For an environment which uses a **local network**:
       - For contracts which have **`workspace = true`**:
         - **build** & **deploy** the contracts, saving the IDs so that on subsequent runs it can instead verify contracts are deployed and update them if needed.
         - **initialize** the contracts: runs any specified `init` commands (see `environments.toml` below)
       - [Beyond the scope of initial grant]: For contracts which instead specify an `environment`, `address`, and `at-ledger-sequence`:
         - **spoon** the specified contract's state, at time of specified ledger sequence, into the current environment's network.
     - For an environment which uses **futurenet**, **testnet**, **mainnet** or some other live network:
       - **check** that the contracts exist on that network. Note: Loam does not yet have plans to help with deploying the contracts. It only checks that you have successfully done so yourself.
     - For all environments:
       - **bind** the contracts
         - run `soroban contract bindings typescript` for each
         - save each generated library to gitignored `packages/*`, part of the [NPM workspace](https://docs.npmjs.com/cli/v10/using-npm/workspaces), using the name specified in `environments.toml`
         - **modify `networks` export** for each, to include all networks specified in `environments.toml`
       - **import** the contracts for use in the frontend. That is, create gitignored `src/contracts/*` files for each, which import the `Contract` class and `networks` object and export an instantiated version for the current environment's network.

### `loam dev`

`loam dev` is a wrapper around `loam build`, but will:

1. Default to `development` environment
2. Automatically watch `contracts/*` and `environments.toml` for changes, and re-run `loam build` when things change

### `loam build` Suggestions

We suggest that each frontend have separate contract dependencies, deployed on separate networks.

So, you should build one version of your frontend for mainnet and host it at the root domain, say, `example.com`. Then build a separate version for testnet and host it at a separate domain, maybe `staging.example.com`.
