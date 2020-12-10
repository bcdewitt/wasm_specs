# Rust Project - WASM + SPECS Example

This project is proof - for my own sake - that [SPECS](https://github.com/amethyst/specs) can be used in a browser environment (some crates can't). It also builds into both a WASM binary file and a native executable. There is just a touch of conditional compilation to be found in the `src/io` folder which shows how this can work.

## Getting Started
NOTE: This project uses **[VS Code Remote - Containers](https://aka.ms/vscode-remote/containers)**. To set it up as intended, you will want to have [Git](https://git-scm.com/), [VS Code](https://code.visualstudio.com/), and [Docker Desktop](https://www.docker.com/products/docker-desktop) installed.

**Linux users:** Update `USER_UID` and `USER_GID` in `.devcontainer/Dockerfile` with your user UID/GID if not 1000 to avoid creating files as root.

After opening the project folder in VS Code, press <kbd>F1</kbd> and select the **Remote-Containers: Open Folder in Container...** command to get started (this may take a while the first time). Once started, you can press <kbd>F5</kbd> to launch and debug or you can run tests and other tasks via **Terminal** -> **Run Task...**.

There are actually two launch configurations, though: "Debug executable" (default) and "Debug web". You can toggle between them in the Run view, which can be found on the [Activity Bar](https://code.visualstudio.com/docs/getstarted/userinterface#_activity-bar) (or use <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd>). [Unfortunately, the debugging story for WebAssembly is still immature](https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger), so the "Debug web" option is rather limited.

### Why Use Remote Containers?
- **Setup is mostly automated** - Compiler, other dependencies, IDE extensions, IDE config, etc. are automated
- **No dependency version conflicts** - If another project uses an older version of a compiler or runtime, it's not an issue
- **Easier cleanup** - The project and its dependencies can be removed from the system in fewer steps
- **Consistent terminal/environment** - Docker aside, no need to handle differences between operating systems
