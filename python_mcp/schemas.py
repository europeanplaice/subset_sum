from __future__ import annotations

from dataclasses import asdict, dataclass
from typing import Any


@dataclass
class TransactionInput:
    id: str
    amount: int
    date: str | None = None
    description: str | None = None

    @classmethod
    def from_dict(cls, raw: dict[str, Any]) -> "TransactionInput":
        return cls(
            id=str(raw["id"]),
            amount=int(raw["amount"]),
            date=_optional_str(raw.get("date")),
            description=_optional_str(raw.get("description")),
        )

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class ReconciliationConfigInput:
    max_key_group_size: int = 5
    max_target_group_size: int = 5
    tolerance: int = 0
    n_candidates: int = 10
    allow_risky_execution: bool = False

    @classmethod
    def from_dict(cls, raw: dict[str, Any] | None) -> "ReconciliationConfigInput":
        raw = raw or {}
        return cls(
            max_key_group_size=int(raw.get("max_key_group_size", 5)),
            max_target_group_size=int(raw.get("max_target_group_size", 5)),
            tolerance=int(raw.get("tolerance", 0)),
            n_candidates=int(raw.get("n_candidates", 10)),
            allow_risky_execution=bool(raw.get("allow_risky_execution", False)),
        )

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


def _optional_str(value: Any) -> str | None:
    if value is None:
        return None
    text = str(value).strip()
    return text or None
