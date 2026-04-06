"""
Domain layer package.
"""

from phenotype_middleware.domain.models import (
    Request,
    Response,
    MiddlewareResult,
    MiddlewareError,
    PipelineError,
    AdapterError,
    ErrorCategory,
)

__all__ = [
    "AdapterError",
    "ErrorCategory",
    "MiddlewareError",
    "MiddlewareResult",
    "PipelineError",
    "Request",
    "Response",
]
