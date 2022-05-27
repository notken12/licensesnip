# 🔑 licensesnip
Tool to automatically add license headers to your source code. Customizable for any language.

Licensesnip is written is Rust and is fast and reliable ⚡.

## 📦 Install

### With Cargo
```bash
cargo install licensesnip
```

## 📜 Usage

In your project's root directory, add a file named `.licensesnip` and write your license header there. Licensesnip will automatically `%FILENAME%` with the file name and `%YEAR%` with the year.

To add license headers to all your source code:

```bash
licensesnip
```
Licensesnip ignores files in your .gitignore file by default.

To remove license headers from all source code:

```bash
licensesnip remove
```

To check if license headers are present in all of your source files:

```bash
licensesnip check
```

You can also specify a specific path or file to modify:

```bash
# Add licenses to src/main.rs
licensesnip src/main.rs
```

```bash
# Remove licenses from src folder
licensesnip src/
```

## ⚙️ Configuration

Find your global Licensesnip config file:
```bash
licensesnip config
```

Create/find the local config file for the current directory:
```bash
licensesnip config -d
```

## ❤️ Contribution

I haven't added builtin support for many languages yet. Please help out and add your favorite languages to `src/base-config.jsonc` and submit a pull request. Thank you!
