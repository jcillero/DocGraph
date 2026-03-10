from __future__ import annotations

from pathlib import Path


# -----------------------------------------------------------------------------
# Root resolution utilities for the portable application
# -----------------------------------------------------------------------------

def resolve_app_root(start: Path | None = None) -> Path:
    """
    Resolve the root of the portable application.

    Works even if called from:
    - tools
    - modules
    - tests
    - scripts

    Strategy:
    Walk upwards until we find a directory containing both:
        system/
        user/

    That directory is the application root.
    """

    if start is None:
        start = Path(__file__).resolve()

    p = start

    # walk up parents
    for parent in [p] + list(p.parents):
        if (parent / "system").exists() and (parent / "user").exists():
            return parent

    raise RuntimeError(
        "App root could not be resolved (missing system/ and user/ directories)."
    )


def resolve_system_root(app_root: Path | None = None) -> Path:
    """
    Return the system/ directory.
    """
    if app_root is None:
        app_root = resolve_app_root()

    return app_root / "system"


def resolve_user_root(app_root: Path | None = None) -> Path:
    """
    Return the user/ directory.
    """
    if app_root is None:
        app_root = resolve_app_root()

    return app_root / "user"