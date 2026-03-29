import json
import random
from datetime import date, timedelta

random.seed(42)

bank_rows = []
ledger_rows = []

start_date = date(2026, 3, 1)

def random_date(start_offset=0, end_offset=30):
    return start_date + timedelta(days=random.randint(start_offset, end_offset))

bank_id_counter = 10000
ledger_id_counter = 20000

# 1-to-1 Matches
for _ in range(150):
    amount = random.randint(1000, 50000)
    d = random_date()
    bank_rows.append({
        "bank_ref": f"BNK-{bank_id_counter}",
        "credit": amount,
        "value_date": str(d + timedelta(days=random.randint(0,2))),
        "narrative": f"Direct Transfer {random.randint(100, 999)}"
    })
    ledger_rows.append({
        "ledger_id": f"INV-{ledger_id_counter}",
        "gross_amount": amount,
        "posted_on": str(d),
        "memo": f"Invoice {random.randint(100, 999)}"
    })
    bank_id_counter += 1
    ledger_id_counter += 1

# Many-to-1 Matches (Multiple ledgers to one bank)
for _ in range(80):
    num_ledgers = random.randint(2, 6)
    amounts = [random.randint(1000, 20000) for _ in range(num_ledgers)]
    total = sum(amounts)
    d = random_date()
    
    bank_rows.append({
        "bank_ref": f"BNK-{bank_id_counter}",
        "credit": total,
        "value_date": str(d + timedelta(days=random.randint(0,2))),
        "narrative": f"Batch Settlement {random.randint(1000, 9999)}"
    })
    bank_id_counter += 1
    
    for amount in amounts:
        ledger_rows.append({
            "ledger_id": f"INV-{ledger_id_counter}",
            "gross_amount": amount,
            "posted_on": str(d),
            "memo": f"Batch Item {random.randint(100, 999)}"
        })
        ledger_id_counter += 1

# Many-to-Many Matches (Simulating complex split batch settlements)
for _ in range(20):
    num_ledgers = random.randint(3, 8)
    amounts = [random.randint(2000, 15000) for _ in range(num_ledgers)]
    total = sum(amounts)
    d = random_date()
    
    # Split the total into 2 or 3 bank deposits
    num_banks = random.randint(2, 3)
    bank_splits = []
    current_total = 0
    for i in range(num_banks - 1):
        split = random.randint(total // (num_banks * 2), total // num_banks)
        bank_splits.append(split)
        current_total += split
    bank_splits.append(total - current_total)
    
    for split in bank_splits:
        bank_rows.append({
            "bank_ref": f"BNK-{bank_id_counter}",
            "credit": split,
            "value_date": str(d + timedelta(days=random.randint(0,2))),
            "narrative": f"Split Settlement {random.randint(1000, 9999)}"
        })
        bank_id_counter += 1
    
    for amount in amounts:
        ledger_rows.append({
            "ledger_id": f"INV-{ledger_id_counter}",
            "gross_amount": amount,
            "posted_on": str(d),
            "memo": f"Split Item {random.randint(100, 999)}"
        })
        ledger_id_counter += 1

# Matches with fee
for _ in range(50):
    amount = random.randint(5000, 50000)
    fee = random.randint(10, 500)
    d = random_date()
    
    bank_rows.append({
        "bank_ref": f"BNK-{bank_id_counter}",
        "credit": amount - fee,
        "value_date": str(d + timedelta(days=random.randint(0,2))),
        "narrative": f"Payment Less Fee {fee}"
    })
    ledger_rows.append({
        "ledger_id": f"INV-{ledger_id_counter}",
        "gross_amount": amount,
        "posted_on": str(d),
        "memo": f"Invoice with expected fee"
    })
    bank_id_counter += 1
    ledger_id_counter += 1

# Unmatched Bank
for _ in range(20):
    bank_rows.append({
        "bank_ref": f"BNK-{bank_id_counter}",
        "credit": random.randint(1000, 20000),
        "value_date": str(random_date()),
        "narrative": "Unknown Deposit"
    })
    bank_id_counter += 1

# Unmatched Ledger
for _ in range(40):
    ledger_rows.append({
        "ledger_id": f"INV-{ledger_id_counter}",
        "gross_amount": random.randint(1000, 20000),
        "posted_on": str(random_date()),
        "memo": "Unpaid Invoice"
    })
    ledger_id_counter += 1

# Adding some uniform amounts to make subset sum confusing
for _ in range(50):
    amount = 10000
    d = random_date()
    if random.choice([True, False]):
        bank_rows.append({
            "bank_ref": f"BNK-{bank_id_counter}",
            "credit": amount,
            "value_date": str(d),
            "narrative": "Standard Subscription"
        })
        bank_id_counter += 1
    else:
        ledger_rows.append({
            "ledger_id": f"INV-{ledger_id_counter}",
            "gross_amount": amount,
            "posted_on": str(d),
            "memo": "Standard Subscription"
        })
        ledger_id_counter += 1

random.shuffle(bank_rows)
random.shuffle(ledger_rows)

with open('bank_rows.json', 'w') as f:
    json.dump(bank_rows, f, indent=2)

with open('ledger_rows.json', 'w') as f:
    json.dump(ledger_rows, f, indent=2)

print(f"Generated {len(bank_rows)} bank rows and {len(ledger_rows)} ledger rows.")