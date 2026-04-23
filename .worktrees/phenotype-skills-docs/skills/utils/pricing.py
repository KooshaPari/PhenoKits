from typing import List, Dict, Any

def compute_unit_price(item: Dict[str, Any]) -> float:
    # simple pricing strategy (same as original): base_price with 10% discount
    base = float(item.get("base_price", 0))
    return base * (1 - 0.1)

def apply_discounts(quotes: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    total = sum(q.get("subtotal", 0) for q in quotes)
    if total > 1000:
        discount_rate = 0.15
    elif total > 500:
        discount_rate = 0.10
    else:
        discount_rate = 0.05
    for q in quotes:
        q["discount"] = discount_rate
        q["final_price"] = q.get("subtotal", 0) * (1 - discount_rate)
    return quotes
