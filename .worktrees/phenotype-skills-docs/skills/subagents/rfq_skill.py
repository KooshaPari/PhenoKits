"""RFQ subagent skill (refactored)
Delegates types to skills.types.phenotype_types and pricing to skills.utils.pricing

@trace SKILL-001: Skill Definition
@trace SKILL-005: Python Skill Support
"""
import logging
from typing import List, Dict, Any

from skills.types.phenotype_types import RFQState
from skills.utils.pricing import compute_unit_price, apply_discounts

logger = logging.getLogger(__name__)


def _parse_request(state: RFQState) -> RFQState:
    rfq_id = state.get("rfq_id", "UNKNOWN")
    logger.info(f"Parsing RFQ request: {rfq_id}")
    items = state.get("items", [])
    if not items:
        state["status"] = "parsing_failed"
        return state
    state["status"] = "request_parsed"
    return state


def _check_inventory(state: RFQState) -> RFQState:
    items = state.get("items", [])
    logger.info(f"Checking inventory for {len(items)} items")
    for item in items:
        item["available"] = item.get("quantity", 0)
    state["status"] = "inventory_checked"
    return state


def _generate_quotes(state: RFQState) -> RFQState:
    items = state.get("items", [])
    logger.info(f"Generating quotes for {len(items)} items")
    quotes: List[Dict[str, Any]] = []
    for item in items:
        unit_price = compute_unit_price(item)
        item_total = unit_price * item.get("quantity", 0)
        quotes.append({
            "sku": item.get("sku"),
            "unit_price": unit_price,
            "quantity": item.get("quantity"),
            "subtotal": item_total,
        })
    state["quotes"] = quotes
    state["status"] = "quotes_generated"
    return state


def _apply_discounts(state: RFQState) -> RFQState:
    quotes = state.get("quotes", [])
    logger.info(f"Applying discounts to {len(quotes)} line items")
    state["quotes"] = apply_discounts(quotes)
    state["status"] = "discounts_applied"
    return state


def _format_quote(state: RFQState) -> RFQState:
    quotes = state.get("quotes", [])
    rfq_id = state.get("rfq_id", "UNKNOWN")
    logger.info(f"Formatting final quote for {rfq_id}")
    final_quote = {
        "rfq_id": rfq_id,
        "line_items": quotes,
        "total": sum(q.get("final_price", 0) for q in quotes),
        "valid_until": "2025-12-26",
    }
    state["selected_quote"] = final_quote
    state["status"] = "quote_ready"
    return state


def create_graph():
    try:
        from langgraph.graph import END, StateGraph  # type: ignore
    except Exception:
        raise RuntimeError("langgraph StateGraph is not available in this runtime")
    graph = StateGraph(RFQState)  # type: ignore
    graph.add_node("parse", _parse_request)
    graph.add_node("inventory", _check_inventory)
    graph.add_node("quote", _generate_quotes)
    graph.add_node("discount", _apply_discounts)
    graph.add_node("format", _format_quote)
    graph.set_entry_point("parse")
    graph.add_edge("parse", "inventory")
    graph.add_edge("inventory", "quote")
    graph.add_edge("quote", "discount")
    graph.add_edge("discount", "format")
    graph.add_edge("format", END)
    return graph


def compile_graph():
    graph = create_graph()
    return graph.compile()


def simulate(state: RFQState) -> RFQState:
    s: RFQState = dict(state)  # shallow copy
    for fn in (_parse_request, _check_inventory, _generate_quotes, _apply_discounts, _format_quote):
        s = fn(s)
    return s


__all__ = ["RFQState", "create_graph", "compile_graph", "simulate"]
