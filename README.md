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
