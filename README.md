# devobs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/lasuillard-s/devobs/graph/badge.svg?token=VlANvU6qUC)](https://codecov.io/gh/lasuillard-s/devobs)
[![PyPI - Version](https://img.shields.io/pypi/v/devobs)](https://pypi.org/project/devobs/)

CLI for obsessed developers.

## ✨ Features

`devobs` aims to provide various development workflow automation tools, such as checking whether matching files exist or detecting changes in the target directory by comparing file hashes before and after running a command.

## 🚀 How to use

Recommended usage is via [pipx](https://pipx.pypa.io/stable/):

```bash
$ pipx run devobs --help
CLI for obsessed developers.

Usage: devobs [OPTIONS] <COMMAND>

Commands:
  check-file-pair  Check for matching file exists
  assert-diff      Detects changes in the target directory by comparing file hashes before and after running a command. Raises an error if any changes are detected
  help             Print this message or the help of the given subcommand(s)

Options:
      --debug
          Enable debug mode. This will increase the verbosity and detail of the logs

      --log-level <LOG_LEVEL>
          Set the log level for the application.

          If `debug` is enabled, the minimum log level will be set to `Debug`.

          [default: INFO]

      --no-colors
          Disable colored output in the logs

      --dry-run
          Dry run mode. If enabled, the application behavior will be changed to not perform any destructive actions

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## 💖 Contributing

Please refer to [CONTRIBUTING.md](./CONTRIBUTING.md) for more information on how to contribute to this project.

## 📜 License

This project is licensed under the MIT License.
