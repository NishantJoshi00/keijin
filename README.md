# Keijin

Simple **Key Generation** tool, provides AES-256 key into `stdout` (without newline)

## Prerequisites

- Rust Environment: [[installation]](<https://www.rust-lang.org/tools/install>)

## Usage

- Build:

  ```bash
  cargo build --release
  ```

- Install (Optional)

  ```bash
  cargo install --path .
  ```

- Key Generation

  ```bash
  # In case of installation
  $ keijin > {target-file}

  # In case
  $ cargo run --release > {target-file}
  ```

  > The key will be stored in the target-file that you specify (without any newline),
  > any further processing that is to be done on this data,
  > must not catch any newline or spaces.
  >
  > **keijin** ensures that, but anything that is done post this is users responsibility
