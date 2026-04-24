"""
Application layer - use cases and orchestration.

This layer contains the MiddlewareChain use case which orchestrates
middleware execution.

xDD Principles:
- Application services orchestrate domain objects
- Use case pattern for business logic
- Transaction script for simple orchestration
"""

from typing import Optional
from phenotype_middleware.domain import Request, Response, MiddlewareResult, PipelineError
from phenotype_middleware.ports import MiddlewarePort


class MiddlewareChain:
    """
    Orchestrates middleware execution.

    This is the primary use case for middleware processing.

    Usage:
        chain = MiddlewareChain()
        chain.add(AuthMiddleware())
        chain.add(LoggingMiddleware())
        result = await chain.handle(request)

    xDD Principles:
    - Single responsibility: orchestrates middleware only
    - Open/closed: add new middleware without modification
    - Dependency inversion: depends on MiddlewarePort abstraction
    """

    def __init__(self) -> None:
        self._middleware: list[MiddlewarePort] = []
        self._error_handler: MiddlewarePort | None = None

    def add(self, middleware: MiddlewarePort) -> "MiddlewareChain":
        """
        Add middleware to the chain.

        Args:
            middleware: MiddlewarePort implementation

        Returns:
            self for fluent API
        """
        self._middleware.append(middleware)
        return self

    def add_error_handler(self, handler: MiddlewarePort) -> "MiddlewareChain":
        """
        Set the error handler for the chain.

        Args:
            handler: MiddlewarePort for error handling

        Returns:
            self for fluent API
        """
        self._error_handler = handler
        return self

    async def handle(self, request: Request) -> MiddlewareResult:
        """
        Handle a request through the middleware chain.

        Args:
            request: Incoming request

        Returns:
            MiddlewareResult from final middleware or error handler

        Raises:
            PipelineError: If chain execution fails
        """
        current_request = request

        for i, middleware in enumerate(self._middleware):
            try:
                result = await middleware.process(current_request)

                if not result.success:
                    return await self._handle_error(
                        result.error or "Middleware failed",
                        current_request,
                        {"middleware_index": i, "middleware": middleware.__class__.__name__}
                    )

                # Update request if modified
                if result.request is not None:
                    current_request = result.request

                # Short-circuit if response is set
                if result.response is not None:
                    return result

            except Exception as e:
                return await self._handle_error(
                    str(e),
                    current_request,
                    {"middleware_index": i, "middleware": middleware.__class__.__name__}
                )

        # No middleware returned a response - return success with request
        return MiddlewareResult.ok(request=current_request)

    async def _handle_error(
        self,
        error: str,
        request: Request,
        context: dict
    ) -> MiddlewareResult:
        """Handle an error from middleware execution."""
        if self._error_handler is not None:
            try:
                error_request = request.with_context("error", error)
                result = await self._error_handler.process(error_request)
                if not result.success:
                    return MiddlewareResult.err(
                        f"Error handler failed: {result.error}",
                        {"original_error": error, **context}
                    )
                return result
            except Exception as e:
                return MiddlewareResult.err(
                    f"Error handler exception: {e!s}",
                    {"original_error": error, **context}
                )

        return MiddlewareResult.err(error, context)
