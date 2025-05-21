# conlock

`conlock` is a container security tool that creates a cryptographic snapshot (a `container.lock` file) of a filesystem and continuously verifies its integrity at runtime. It only allows an executable entrypoint to run if the filesystem matches the manifest specified in the `container.lock` file, providing strong protection against unauthorized filesystem changes.

---

## Features

- Recursively scan directories and hash files using **xxHash** for speed.
- Exclude paths or files using glob patterns.
- Generate a `container.lock` manifest file describing the expected filesystem state.
- Run a specified command only if the filesystem matches the manifest.
- Periodically verify filesystem integrity at configurable intervals (e.g., every 1, 5, or 15 minutes).
- Terminate the monitored executable and itself immediately if any integrity violation is detected.

---

## Usage

### 1. Generate a `container.lock` File

```bash
conlock lock /path/to/directory --exclude "tmp/*" --exclude "*.log"
```

This command scans `/path/to/directory`, excluding any paths matching the specified patterns, and creates a `container.lock` file representing the current filesystem state.

### 2. Verify and Run an Executable Using the Lock File

```bash
conlock verify container.lock -- /path/to/executable arg1 arg2
```

This command reads the `container.lock` file, launches the executable with the specified arguments, and monitors the filesystem at intervals. If the filesystem deviates from the manifest, both the executable and `conlock` will terminate.

### 3. Optional Flags

- `--interval <seconds>`: Set the verification interval (default is 60 seconds).
- `--lockfile <file>`: Specify a custom lock file location.
- `--exclude <pattern>`: Glob pattern(s) to exclude during scanning.

---

## How It Works

- **Lock Generation:**  
  `conlock` recursively scans the specified directory, hashes files with xxHash, and writes file metadata and hashes into a `container.lock` file.

- **Runtime Verification:**  
  `conlock` starts the executable entrypoint, then periodically rescans the filesystem, verifying that every file matches the hash in `container.lock`. Any unexpected changes cause immediate shutdown.

---

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/hunterkirk/conlock.git
   cd conlock
   ```

2. Build with Cargo:
   ```bash
   cargo build --release
   ```

3. Run the binary located at `conlock`.

---


## Disclaimer

`conlock` is designed as a security monitoring tool. Use it carefully and review your lock files regularly to ensure they reflect your desired filesystem state.

---
