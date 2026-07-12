#!/usr/bin/env python3
"""Create missing Refinery migration files from reviewed Atlas migrations.

Atlas owns dev schema diffs in db/migrations. Refinery owns production/runtime
execution in db/refinery_migrations. This helper maps the sorted Atlas migration
sequence to Refinery's V<N>__name.sql convention without overwriting reviewed
Refinery files.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ATLAS_DIR = ROOT / "db" / "migrations"
REFINERY_DIR = ROOT / "db" / "refinery_migrations"


def atlas_migrations() -> list[Path]:
    return sorted(path for path in ATLAS_DIR.glob("*.sql") if path.is_file())


def refinery_versions() -> dict[int, Path]:
    versions: dict[int, Path] = {}
    for path in sorted(REFINERY_DIR.glob("V*.sql")):
        match = re.match(r"^V(\d+)__.+\.sql$", path.name)
        if not match:
            print(f"error: invalid Refinery migration name: {path}", file=sys.stderr)
            sys.exit(1)

        version = int(match.group(1))
        if version in versions:
            print(
                f"error: duplicate Refinery migration version V{version}: "
                f"{versions[version]} and {path}",
                file=sys.stderr,
            )
            sys.exit(1)
        versions[version] = path

    if versions:
        expected = set(range(1, max(versions) + 1))
        missing = sorted(expected - set(versions))
        if missing:
            missing_versions = ", ".join(f"V{version}" for version in missing)
            print(f"error: missing Refinery version(s): {missing_versions}", file=sys.stderr)
            sys.exit(1)

    return versions


def migration_slug(atlas_path: Path) -> str:
    stem = atlas_path.stem
    match = re.match(r"^\d+_(.+)$", stem)
    name = match.group(1) if match else stem
    slug = re.sub(r"[^a-zA-Z0-9]+", "_", name).strip("_").lower()
    return slug or "migration"


def main() -> int:
    atlas_files = atlas_migrations()
    if not atlas_files:
        print("error: no Atlas SQL migrations found in db/migrations", file=sys.stderr)
        return 1

    REFINERY_DIR.mkdir(parents=True, exist_ok=True)
    existing = refinery_versions()
    created: list[Path] = []

    if len(existing) > len(atlas_files):
        print(
            "error: more Refinery migrations exist than Atlas migrations; "
            "inspect db/refinery_migrations manually",
            file=sys.stderr,
        )
        return 1

    for index, atlas_path in enumerate(atlas_files, start=1):
        if index in existing:
            print(f"skip V{index}: {existing[index].name} already exists")
            continue

        target = REFINERY_DIR / f"V{index}__{migration_slug(atlas_path)}.sql"
        if target.exists():
            print(f"error: target already exists: {target}", file=sys.stderr)
            return 1

        target.write_text(atlas_path.read_text(), encoding="utf-8")
        created.append(target)
        print(f"created {target.relative_to(ROOT)} from {atlas_path.relative_to(ROOT)}")

    if not created:
        print("Refinery migrations are already in sync with Atlas migrations")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
