from typing import Protocol, Dict, Any

class InventoryPort(Protocol):
    def check(self, sku: str) -> int: ...

class PaymentPort(Protocol):
    def charge(self, customer_id: str, amount: float) -> Dict[str, Any]: ...

class ShippingPort(Protocol):
    def create_label(self, order_id: str, address: Dict[str, Any]) -> Dict[str, Any]: ...
