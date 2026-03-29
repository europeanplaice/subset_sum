from __future__ import annotations

from decimal import Decimal, InvalidOperation, ROUND_HALF_UP
from typing import Any

from schemas import TransactionInput


def parse_minor_units(value: Any, scale: int = 100) -> int:
    if isinstance(value, int):
        return value
    if isinstance(value, float):
        value = str(value)

    try:
        decimal_value = Decimal(str(value).replace(",", "").strip())
    except (InvalidOperation, AttributeError) as exc:
        raise ValueError(f"Invalid amount: {value!r}") from exc

    quantized = (decimal_value * scale).quantize(Decimal("1"), rounding=ROUND_HALF_UP)
    return int(quantized)


def normalize_transaction_rows(
    rows: list[dict[str, Any]],
    *,
    id_field: str = "id",
    amount_field: str = "amount",
    date_field: str = "date",
    description_field: str = "description",
    amounts_are_minor_units: bool = True,
) -> list[dict[str, Any]]:
    normalized: list[dict[str, Any]] = []

    for index, row in enumerate(rows):
        raw_id = row.get(id_field)
        tx_id = str(raw_id).strip() if raw_id not in (None, "") else f"row-{index + 1}"
        raw_amount = row.get(amount_field)
        if raw_amount is None:
            raise ValueError(f"Missing amount at row {index + 1}")

        amount = int(raw_amount) if amounts_are_minor_units else parse_minor_units(raw_amount)
        tx = TransactionInput(
            id=tx_id,
            amount=amount,
            date=_optional_text(row.get(date_field)),
            description=_optional_text(row.get(description_field)),
        )
        normalized.append(tx.to_dict())

    return normalized


def _optional_text(value: Any) -> str | None:
    if value is None:
        return None
    text = str(value).strip()
    return text or None
