# Component Name

Component description.

## Overview

This component provides $tool_name compatibility for SigmaOS.

## Features

- Feature 1
- Feature 2

## Usage

```rust
use sigmaos_system_util::SystemUtil;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut util = SystemUtil::new()?;
    util.init().await?;
    Ok(())
}
```

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## License

MIT License - see LICENSE file for details

## Inspiration

- $tool_name utility
- system tools
- desktop environment
