# metagross

A UCI chess engine written in Rust.

## Architecture

- **rules/** - Chess rules layer. Defines the `Position` trait and provides implementations. Knows nothing about UCI or engine strategy.
- **engine/** - Move selection and search. Implements the `Engine` trait to choose moves. Knows nothing about UCI protocol details.
- **uci/** - Universal Chess Interface protocol handler. Manages communication with UCI clients. Knows nothing about move search or chess strategy.

## Module Map

```
src/
├── main.rs              - Entry point: builds RandomEngine, runs UCI loop
├── lib.rs               - Library root
├── rules/
│   ├── mod.rs           - Position trait and Move type
│   └── shakmaty.rs      - Position implementation using shakmaty
├── engine/
│   ├── mod.rs           - Engine trait and Limits
│   └── random.rs        - RandomEngine implementation
└── uci/
    ├── mod.rs           - UCI run loop
    ├── command.rs       - Command parsing
    ├── response.rs      - Response formatting
    └── session.rs       - Session state management
```

## How to Run

Build the engine:

```bash
cargo build --release
```

Run in UCI mode:

```bash
cargo run --release
```

The engine accepts UCI commands on stdin and writes responses to stdout.

## Lichess

To use this engine on Lichess:

1. Create a BOT account on Lichess (separate from your normal account)
2. Generate an API token with bot permissions
3. Use [lichess-bot](https://github.com/lichess-bot-devs/lichess-bot) bridge to connect this UCI engine to your Lichess bot account
4. Configure lichess-bot to run `metagross` as the engine binary
