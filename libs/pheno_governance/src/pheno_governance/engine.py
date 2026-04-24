"""Governance engine implementation."""
from __future__ import annotations
import hashlib, json, re
from dataclasses import dataclass, asdict
from datetime import datetime
from enum import Enum, auto
from pathlib import Path
from typing import Any

class PolicyType(Enum): SECURITY = auto(); COMPLIANCE = auto(); COST = auto(); QUALITY = auto(); CUSTOM = auto()
class EnforcementMode(Enum): AUDIT = auto(); WARN = auto(); BLOCK = auto(); REMEDIATE = auto()

@dataclass
class Policy:
    id: str; name: str; description: str; type: PolicyType; rules: list[dict]; enforcement: EnforcementMode; enabled: bool = True
    def compute_hash(self) -> str: return hashlib.sha256(json.dumps(asdict(self), sort_keys=True).encode()).hexdigest()

@dataclass
class ComplianceReport:
    timestamp: datetime; resource: str; policy_id: str; compliant: bool; violations: list[dict]; severity: str

@dataclass
class GovernanceDecision:
    timestamp: datetime; action: str; resource: str; actor: str; decision: str; policy_applied: str; reasoning: str

class GovernanceEngine:
    def __init__(self, policies_path: Path | None = None) -> None:
        self._policies_path = policies_path or Path(".governance"); self._policies_path.mkdir(parents=True, exist_ok=True)
        self._policies: dict[str, Policy] = {}; self._load_policies()
    
    def _load_policies(self) -> None:
        for policy_file in self._policies_path.glob("*.json"):
            try:
                data = json.loads(policy_file.read_text()); policy = Policy(**data); self._policies[policy.id] = policy
            except: pass
    
    def create_policy(self, name: str, description: str, policy_type: PolicyType, rules: list[dict], enforcement: EnforcementMode) -> Policy:
        policy_id = f"policy_{name.lower().replace(' ', '_')}"
        policy = Policy(id=policy_id, name=name, description=description, type=policy_type, rules=rules, enforcement=enforcement)
        self._policies[policy_id] = policy
        (self._policies_path / f"{policy_id}.json").write_text(json.dumps(asdict(policy), indent=2, default=str))
        return policy
    
    def get_policy(self, policy_id: str) -> Policy | None: return self._policies.get(policy_id)
    def list_policies(self, enabled_only: bool = True) -> list[Policy]:
        policies = list(self._policies.values())
        if enabled_only: policies = [p for p in policies if p.enabled]
        return policies
    
    def _evaluate_rule(self, rule: dict[str, Any], context: dict[str, Any]) -> tuple[bool, str | None]:
        rule_type = rule.get("type", "match")
        if rule_type == "match":
            key, value = rule.get("key"), rule.get("value")
            if key in context: return context[key] == value, None
            return False, f"Key '{key}' not found"
        elif rule_type == "regex":
            key, pattern = rule.get("key"), rule.get("pattern")
            if key in context and isinstance(context[key], str):
                return re.search(pattern, context[key]) is not None, None
        return False, f"Unknown rule type: {rule_type}"
    
    def check_compliance(self, resource: str, context: dict[str, Any]) -> ComplianceReport:
        violations = []; max_severity = "none"
        for policy in self.list_policies(enabled_only=True):
            for rule in policy.rules:
                passed, error = self._evaluate_rule(rule, context)
                if not passed:
                    violation = {"policy_id": policy.id, "policy_name": policy.name, "rule": rule, "error": error, "severity": rule.get("severity", "medium")}
                    violations.append(violation)
                    severity_order = {"low": 1, "medium": 2, "high": 3, "critical": 4}
                    if severity_order.get(violation["severity"], 0) > severity_order.get(max_severity, 0): max_severity = violation["severity"]
        return ComplianceReport(timestamp=datetime.utcnow(), resource=resource, policy_id="governance", compliant=len(violations) == 0, violations=violations, severity=max_severity)
    
    def make_decision(self, action: str, resource: str, actor: str, context: dict[str, Any]) -> GovernanceDecision:
        report = self.check_compliance(resource, context)
        decision = "DENY" if (not report.compliant and report.severity in ["high", "critical"]) else "ALLOW"
        return GovernanceDecision(timestamp=datetime.utcnow(), action=action, resource=resource, actor=actor, decision=decision, policy_applied=report.policy_id, reasoning=f"{len(report.violations)} violations" if report.violations else "Compliant")

_default_engine: GovernanceEngine | None = None
def get_engine() -> GovernanceEngine:
    global _default_engine
    if _default_engine is None: _default_engine = GovernanceEngine()
    return _default_engine
