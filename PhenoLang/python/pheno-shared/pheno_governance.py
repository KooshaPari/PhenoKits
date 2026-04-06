"""Pheno Governance - Policy and compliance."""
from dataclasses import dataclass, asdict
from datetime import datetime
from enum import Enum, auto
import hashlib
import json

class PolicyType(Enum):
    SECURITY = auto()
    COMPLIANCE = auto()
    COST = auto()
    QUALITY = auto()

class EnforcementMode(Enum):
    AUDIT = auto()
    WARN = auto()
    BLOCK = auto()
    REMEDIATE = auto()

@dataclass
class Policy:
    id: str
    name: str
    description: str
    type: PolicyType
    rules: list
    enforcement: EnforcementMode
    enabled: bool = True
    
    def compute_hash(self):
        return hashlib.sha256(json.dumps(asdict(self), sort_keys=True).encode()).hexdigest()

class GovernanceEngine:
    def __init__(self):
        self._policies = {}
    
    def create_policy(self, name, description, policy_type, rules, enforcement):
        policy_id = f"policy_{name.lower().replace(' ', '_')}"
        policy = Policy(
            id=policy_id,
            name=name,
            description=description,
            type=policy_type,
            rules=rules,
            enforcement=enforcement
        )
        self._policies[policy_id] = policy
        return policy
    
    def get_policy(self, policy_id):
        return self._policies.get(policy_id)
    
    def list_policies(self, enabled_only=True):
        policies = list(self._policies.values())
        if enabled_only:
            policies = [p for p in policies if p.enabled]
        return policies
    
    def check_compliance(self, resource, context):
        violations = []
        for policy in self.list_policies():
            for rule in policy.rules:
                rule_type = rule.get("type", "match")
                if rule_type == "match":
                    key = rule.get("key")
                    value = rule.get("value")
                    if key in context and context[key] != value:
                        violations.append({"policy_id": policy.id, "rule": rule})
        return {"compliant": len(violations) == 0, "violations": violations}

_engine = None

def get_engine():
    global _engine
    if _engine is None:
        _engine = GovernanceEngine()
    return _engine

__all__ = ["Policy", "PolicyType", "EnforcementMode", "GovernanceEngine", "get_engine"]
