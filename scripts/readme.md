# Build Scripts

Helper scripts for Linux (WSL) and Windows wheel builds live here. Each script switches to the repository root before running, so you can invoke them from anywhere inside the repo.

## Linux — `build_linux.sh`

```bash
./scripts/build_linux.sh
```

- Requires Docker CLI on WSL2 (Docker Desktop or equivalent).
- Pulls `ghcr.io/pyo3/maturin` by default and builds a manylinux2014-compatible wheel for CPython ≥ 3.8 (abi3).
- Tunable environment variables:
  - `TARGET_ARCH` (default: `x86_64-unknown-linux-gnu`).
  - `COMPAT_LEVEL` (default: `manylinux_2_17`).
  - `OUTPUT_DIR` (default: `dist`).
  - `DOCKER_IMAGE` (default: `ghcr.io/pyo3/maturin`).
  - `SKIP_PULL=1` skips the `docker pull` step if the image is already local.
  - `EXTRA_ARGS="--sdist"` passes additional `maturin build` flags (array-friendly syntax).

## Windows — `build_windows.ps1`

```powershell
pwsh ./scripts/build_windows.ps1
```

- Assumes Visual Studio Build Tools (Desktop development with C++) and a CPython interpreter (e.g., `py -3.12`).
- Runs `maturin build --release --features python --target x86_64-pc-windows-msvc --interpreter python3.12 --out dist` by default.
- Parameters:
  - `-Interpreter python3.12` — choose which CPython to point PyO3 at for import libs.
  - `-Target x86_64-pc-windows-msvc` — adjust if you ever produce other Windows targets.
  - `-OutputDir dist` — override the wheel output directory.
  - `-ExtraArgs @('--sdist')` — append extra `maturin` arguments.

## Quick Verification

After either script finishes, smoke-test the wheel on the same OS:

```bash
pip install dist/dpss-*.whl
python -c "import dpss; print(dpss.__version__)"
```

Translate to PowerShell (`py -3.12 -m pip install dist\dpss-*.whl`) when testing on Windows.
