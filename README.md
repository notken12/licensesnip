# 🔑 licensesnip
Tool to automatically add license headers to your source code. Customizable for any language.

Licensesnip is written is Rust and is fast and reliable ⚡.

## 📦 Install

### With Cargo
```bash
cargo install licensesnip
```

## 📜 Usage

In your project's root directory, add a file named `.licensesnip` and write your license header there. Licensesnip will automatically replace `%FILENAME%` with the file name and `%YEAR%` with the year.

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
licensesnip remove src/
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

Example configuration:
```json
{
  "use_gitignore": true,
  "file_types": {
    "js,mjs,ts,cjs,jsx,tsx": {
      "before_line": "// "
    },
    "vue,html": {
      "before_block": "<!--",
      "before_line": "  ",
      "after_block": "-->"
    },
    "rs": {
      "before_line": "// "
    },
    "c": {
      "enable": false
    }
  }
}
```

To configure a language just specify how the comments for that language work. Supported properties are `before_line`, `after_line`, `before_block`, and `after_block`. To disable adding licenses to a filetype, set `enable` to false.

## ❤️ Contribution

I haven't added builtin support for many languages yet. Please help out and add your favorite languages to `src/base-config.jsonc` and submit a pull request. Thank you!
