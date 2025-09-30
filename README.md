# OpenFootball 2026

A complete football management game, inspired by Football Manager.

## Prerequisites

- Node.js >= 24
- Rust & Cargo >= 1.90

## Installation

```bash
# Clone the repository
git https://github.com/vvasconceloss/openfootball.git

# Navigate to the project folder
cd openfootball

# Install dependencies
cd tauri
pnpm install
```

## Running the Application

```bash
# Run Tauri dev
cd src-tauri
pnpm run tauri dev
```

## Project Structure

```
openfootball/
├── application/        # use-cases
├── core/               # entities, enums, traits
├── persistence/        # sqlx schemas, repository implementations
├── simulation/         # match engine, RNG, event generation
├── tauri/              # tauri application
│   ├── src-tauri/      # tauri backend (rust) code
│   │    ├── main.rs    # tauri main entry
│   │    └── lib.rs     # optional library code
├── Cargo.toml
├── .gitignore
├── LICENSE
└── README.md           
```

## Contribution

1. Fork the project
2. Create a branch: `git checkout -b feature/new-feature`
3. Commit your changes: `git commit -m 'Add new feature'`
4. Push to the remote repository: `git push origin feature/new-feature`
5. Open a Pull Request

## License

MIT © [Victor Olimpio Vasconcelos](https://github.com/vvasconceloss)
</br>
Check [LICENSE](./LICENSE) for more information.