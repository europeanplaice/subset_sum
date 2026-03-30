import pytest
from decimal import Decimal
from normalize import parse_minor_units, normalize_transaction_rows, _optional_text

def test_parse_minor_units():
    assert parse_minor_units(123) == 123
    assert parse_minor_units("12.34") == 1234
    assert parse_minor_units("1,234.56") == 123456
    assert parse_minor_units(12.34) == 1234
    assert parse_minor_units("12.345") == 1235 # ROUND_HALF_UP

    with pytest.raises(ValueError):
        parse_minor_units("invalid")

def test_optional_text():
    assert _optional_text(None) is None
    assert _optional_text("  ") is None
    assert _optional_text(" foo ") == "foo"

def test_normalize_transaction_rows():
    rows = [
        {"id": "1", "amount": 100, "date": "2023-01-01", "description": "test"},
        {"amount": 200}, # missing id, date, description
        {"id": "", "amount": "300"} # empty id
    ]
    
    normalized = normalize_transaction_rows(rows)
    assert len(normalized) == 3
    assert normalized[0] == {"id": "1", "amount": 100, "date": "2023-01-01", "description": "test"}
    assert normalized[1] == {"id": "row-2", "amount": 200, "date": None, "description": None}
    assert normalized[2] == {"id": "row-3", "amount": 300, "date": None, "description": None}

    # Test amounts_are_minor_units=False
    rows2 = [{"id": "1", "amount": "12.34"}]
    normalized2 = normalize_transaction_rows(rows2, amounts_are_minor_units=False)
    assert normalized2[0]["amount"] == 1234

    # Test missing amount
    with pytest.raises(ValueError):
        normalize_transaction_rows([{"id": "1"}])
