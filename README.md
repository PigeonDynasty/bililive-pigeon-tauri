# Tauri + Svelte + Typescript

This template should help get you started developing with Tauri, Svelte and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Tips
1. You may need to enter the following code in the terminal to get it to work in Macos.
```
sudo xattr -d com.apple.quarantine /Applications/Bililive\ Pigeon.app
```
2. The relevant files are stored in `~/Documents/BililivePigeon`.
## Development
You need `Node.js` `Pnpm` `Rust`,please install these by yourself.
```
# Install dependencies
pnpm install

# Run in dev mode
pnpm tauri dev
```

For more information, please see [https://tauri.app/]()

## Plugins System
You can review the following documents to see the plug-in implementation.
- src/store/plugin.ts
- src-tauri/src/plugin.rs

**PS**

I haven't checked the usability of the plug-in system too much, so if you have a problem, you can submit an issue or pull request if you can.

**Please do not call me for problems caused by plug-ins.**