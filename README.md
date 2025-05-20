# Kagi Assistant Desktop

A desktop application for [Kagi Assistant](https://kagi.com/assistant) built with Tauri.

* Note: This Application has some weird redirection flows due to kagi.com/assistant redirecting to an documentation page instead of the signin page if you're not signed in but after the first time you sign in, the program will remember that you've signed in and will autodirect you to assistant. If you're having issues with redirection or auth, please wait for kagi change this from planned to resolved so I can majorly simplify this applicaiton https://kagifeedback.org/d/7124-kagi-assistant-redirect-to-signin-page-instead-of-helpkagicom

![image](https://github.com/user-attachments/assets/4946da7f-94a8-41ff-b3ce-9ab0e8f07d4c)

## Install

### Binaries (Mac, Linux, Windows)

Download from https://github.com/0xGingi/kagi-assistant-desktop/releases/latest

### Arch Linux (AUR)

AUR Package: https://aur.archlinux.org/packages/kagi-assistant-desktop-git

```bash
paru -S kagi-assistant-desktop-git
```

## Build

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Bun](https://bun.sh/)

### Building

```bash
bun install

bun run tauri build --bundles
```

This will build to `src-tauri/target/release/bundle`.
