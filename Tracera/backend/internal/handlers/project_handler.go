package handlers

import (
	"github.com/kooshapari/tracertm-backend/internal/cache"
	"github.com/kooshapari/tracertm-backend/internal/nats"
	"github.com/kooshapari/tracertm-backend/internal/services"
)

// ProjectHandler handles HTTP requests for project operations.
type ProjectHandler struct {
	projectService services.ProjectService // Service layer for operations
	cache          cache.Cache
	publisher      *nats.EventPublisher
	binder         RequestBinder
}

// NewProjectHandler creates a new ProjectHandler instance.
func NewProjectHandler(
	projectService services.ProjectService,
	cache cache.Cache,
	publisher *nats.EventPublisher,
	binder RequestBinder,
) *ProjectHandler {
	return &ProjectHandler{
		projectService: projectService,
		cache:          cache,
		publisher:      publisher,
		binder:         binder,
	}
}
