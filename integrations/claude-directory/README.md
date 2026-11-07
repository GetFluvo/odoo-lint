# odoo-lint skill for Claude

Teaches Claude to lint and fix Odoo addons with
[odoo-lint](https://github.com/GetFluvo/odoo-lint) (`odl`): pylint-odoo's
checks, OCA's manifest, XML and PO checks, translation files Odoo cannot load,
and upgrade checks for newer Odoo versions.

This is the directory edition: one skill that tells Claude when to run `odl`,
how to treat findings and fixes, and how Odoo loads translations. It runs
nothing by itself and sends no data anywhere; Claude runs `odl` in your
project when the skill applies.

## Requirements

Install `odl` first:

```sh
uv tool install odoo-linter
# or
pip install odoo-linter
```

## Full plugin

The full plugin adds an MCP server and a hook that lints every addon file
after Claude edits it. Install it from the project's own marketplace:

```text
/plugin marketplace add GetFluvo/odoo-lint
/plugin install odoo-lint@odoo-lint
```

## Configuration

`odl` reads `[tool.odoo-lint]` from `pyproject.toml`, or `odoo-lint.toml`;
see [Configuration](https://odoo-lint.readthedocs.io/en/latest/configuration.html).

## License

MIT
