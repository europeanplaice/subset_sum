# Build Scripts

Helper scripts for Linux (WSL) and Windows wheel builds live here. Each script switches to the repository root before running, so you can invoke them from anywhere inside the repo.

## Linux — `build_linux.sh`

```bash
./scripts/build_linux.sh
```

- Requires Docker CLI on WSL2 (Docker Desktop or equivalent).
- Pulls `ghcr.io/pyo3/maturin` by default and builds a manylinux2014-compatible wheel **and** sdist for CPython ≥ 3.8 (abi3).
- Tunable environment variables:
  - `TARGET_ARCH` (default: `x86_64-unknown-linux-gnu`).
  - `COMPAT_LEVEL` (default: `manylinux_2_17`).
  - `OUTPUT_DIR` (default: `dist`).
  - `DOCKER_IMAGE` (default: `ghcr.io/pyo3/maturin`).
  - `SKIP_PULL=1` skips the `docker pull` step if the image is already local.
  - `EXTRA_ARGS="--no-sdist"` passes additional `maturin build` flags (array-friendly syntax) when you truly need to skip the sdist.

## Windows — `build_windows.ps1`

```powershell
pwsh ./scripts/build_windows.ps1
```

- Assumes Visual Studio Build Tools (Desktop development with C++) and a CPython interpreter (e.g., `py -3.12`).
- Runs `maturin build --release --features python --target x86_64-pc-windows-msvc --interpreter python3.12 --out dist` by default (wheel only).
- Parameters:
  - `-Interpreter python3.12` — choose which CPython to point PyO3 at for import libs.
  - `-Target x86_64-pc-windows-msvc` — adjust if you ever produce other Windows targets.
  - `-OutputDir dist` — override the wheel output directory.
  - `-ExtraArgs @('--sdist')` — append extra `maturin` arguments (use `--sdist` if you need Windows to emit it too).

## Quick Verification

After either script finishes, smoke-test the wheel on the same OS:

```bash
pip install dist/dpss-*.whl
python -c "import dpss; print(dpss.__version__)"
```

Translate to PowerShell (`py -3.12 -m pip install dist\dpss-*.whl`) when testing on Windows.

## Publishing with Twine

After both Linux (`build_linux.sh`) and Windows (`build_windows.ps1`) runs drop artifacts into `dist/`, upload them with Twine:

```bash
python -m pip install --upgrade twine
python -m twine check dist/*
python -m twine upload dist/*
```

- Start with TestPyPI via `--repository testpypi` (or `TWINE_REPOSITORY=testpypi`), then switch to the real PyPI once satisfied.
- Pass credentials through `TWINE_USERNAME`/`TWINE_PASSWORD`, an API token (`__token__`), or `~/.pypirc` entries so secrets never land in shell history.
- Limit the glob (e.g., `dist/dpss-0.1.0-*`) if you only want to publish the fresh wheels and leave older files untouched.
