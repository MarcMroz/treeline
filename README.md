# Treeline

**Local-first personal finance**

Your financial data stays on your computer in a DuckDB database. No cloud accounts, no subscriptions, no data harvesting.

[Download](https://treeline.money/download) · [Documentation](https://docs.treeline.money) · [Discord](https://discord.gg/EcNvBnSft5)

> **Beta**: Treeline is in active development. Back up your data and expect breaking changes.

## Quick Start

Download the desktop app from [treeline.money/download](https://treeline.money/download), or install the CLI:

```bash
# macOS / Linux
curl -fsSL https://treeline.money/install.sh | sh

# Windows (PowerShell)
irm https://treeline.money/install.ps1 | iex
```

See the [Getting Started guide](https://docs.treeline.money/getting-started/installation/) for details.

## Repository Structure

| Directory | Description |
|-----------|-------------|
| `desktop/` | Desktop app (Tauri + Svelte) |
| `core/` | Rust core library |
| `cli/` | Rust CLI (`tl` command) |
| `sdk/` | TypeScript SDK for plugins ([npm](https://www.npmjs.com/package/@treeline-money/plugin-sdk)) |
| `template/` | Starter template for new plugins |
| `docs/` | Documentation site ([docs.treeline.money](https://docs.treeline.money)) |
| `plugins.json` | Registry of community plugins |

## Documentation

- [Installation](https://docs.treeline.money/getting-started/installation/)
- [CLI Reference](https://docs.treeline.money/cli/)
- [Building Plugins](https://docs.treeline.money/plugins/creating-plugins/)
- [Database Schema](https://docs.treeline.money/reference/database-schema/)

## AI / MCP Extension

Treeline ships as a [Desktop Extension](https://docs.treeline.money/ai-agents/mcp-server/) for Claude Desktop. Install the `.mcpb` file from the [latest release](https://github.com/treeline-money/treeline/releases/latest), or use the CLI directly as an MCP server.

### Usage Examples

**Analyze your spending:**
> "What are my top spending categories this month?"

**Find large transactions:**
> "Show me all transactions over $200 this week"

**Organize with tags:**
> "Tag all Uber and Lyft transactions as transport"

**Track your net worth:**
> "What's my net worth and how has it changed over the last 6 months?"

**Discover recurring charges:**
> "What are my recurring monthly charges?"

### Testing with Demo Mode

Reviewers and new users can try the MCP tools without real bank data:

```bash
tl demo on    # Load sample financial data
tl demo off   # Remove sample data
```

## Privacy Policy

Treeline is local-first. All financial data stays on your device in a DuckDB database. No telemetry, no analytics, no data collection. See the full [Privacy Policy](https://treeline.money/privacy).

## Support

- [GitHub Issues](https://github.com/treeline-money/treeline/issues)
- [Discord](https://discord.gg/EcNvBnSft5)
- [Documentation](https://docs.treeline.money)

## Contributing

See the [Contributing guide](https://docs.treeline.money/contributing/) for development setup.

## License

MIT
