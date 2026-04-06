from typing import Dict, Any
from skills.ports.ports import InventoryPort, PaymentPort, ShippingPort

class InMemoryInventory:
    def __init__(self, stock: Dict[str,int]):
        self.stock = stock
    def check(self, sku: str) -> int:
        return self.stock.get(sku, 0)

class SimplePayment:
    def charge(self, customer_id: str, amount: float) -> Dict[str,Any]:
        return {"status":"charged","customer":customer_id,"amount":amount}

class DummyShipping:
    def create_label(self, order_id: str, address: Dict[str,Any]) -> Dict[str,Any]:
        return {"tracking_number":f"TRACK-{order_id}","carrier":"DUMMY","cost":10.0}
