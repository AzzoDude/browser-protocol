#!/usr/bin/env python3
"""Publish the workspace crates to crates.io, skipping versions that already exist.

Crates are published in dependency order (`browser-protocol-macros` before
`browser-protocol`), and the script waits for crates.io to index each crate
before publishing the next one. Skipping already-published versions makes the
release workflow safe to re-run after a partial failure.

Authentication is handled by Cargo: set `CARGO_REGISTRY_TOKEN` (or use the
short-lived token produced by trusted publishing in CI).
"""

import argparse
import os
import re
import subprocess
import sys
import time
import urllib.error
import urllib.request

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)

# (crate name, Cargo.toml path) in publish order.
CRATES = [
    ("browser-protocol-macros", os.path.join(PROJECT_ROOT, "macros", "Cargo.toml")),
    ("browser-protocol", os.path.join(PROJECT_ROOT, "Cargo.toml")),
]

INDEX_POLL_ATTEMPTS = 30
INDEX_POLL_SECONDS = 5


def package_version(cargo_toml: str) -> str:
    with open(cargo_toml, "r", encoding="utf-8") as f:
        for line in f:
            match = re.match(r'^version\s*=\s*"([^"]+)"', line)
            if match:
                return match.group(1)
    raise RuntimeError(f"no package version found in {cargo_toml}")


def version_exists(name: str, version: str) -> bool:
    url = f"https://crates.io/api/v1/crates/{name}/{version}"
    request = urllib.request.Request(url, headers={"User-Agent": "browser-protocol-release"})
    try:
        with urllib.request.urlopen(request, timeout=30):
            return True
    except urllib.error.HTTPError as err:
        if err.code == 404:
            return False
        raise


def publish(name: str) -> None:
    print(f"publishing {name} ...")
    subprocess.run(["cargo", "publish", "-p", name], check=True)


def wait_until_indexed(name: str, version: str) -> None:
    for _ in range(INDEX_POLL_ATTEMPTS):
        if version_exists(name, version):
            return
        time.sleep(INDEX_POLL_SECONDS)
    print(f"warning: {name} {version} is not visible on crates.io yet", file=sys.stderr)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dry-run", action="store_true", help="print the plan without publishing")
    args = parser.parse_args()

    for name, cargo_toml in CRATES:
        version = package_version(cargo_toml)
        if version_exists(name, version):
            print(f"skipping {name} {version} (already published)")
            continue
        if args.dry_run:
            print(f"would publish {name} {version}")
            continue
        publish(name)
        wait_until_indexed(name, version)
    return 0


if __name__ == "__main__":
    sys.exit(main())
