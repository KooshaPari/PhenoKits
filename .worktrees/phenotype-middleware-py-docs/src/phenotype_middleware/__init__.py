"""
phenotype-middleware
~~~~~~~~~~~~~~~~~~~~

Python middleware patterns following hexagonal architecture.

xDD Methodologies:
- TDD: Test-driven development
- BDD: Behavior-driven with scenario naming
- DDD: Domain-driven design with bounded contexts
- CDD: Contract-driven with port/adapter verification

Example:
    >>> from phenotype_middleware import MiddlewareChain
    >>> chain = MiddlewareChain()
    >>> chain.add(my_middleware)
    >>> result = await chain.handle(request)
"""

__version__ = "0.1.0"

from phenotype_middleware.domain.models import Request, Response, MiddlewareResult
from phenotype_middleware.ports import MiddlewarePort

__all__ = [
    "MiddlewarePort",
    "MiddlewareResult",
    "Request",
    "Response",
    "__version__",
]
