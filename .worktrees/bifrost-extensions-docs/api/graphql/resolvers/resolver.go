// Package resolvers implements GraphQL resolvers for the Bifrost API.
package resolvers

import (
	"context"
	"log/slog"
	"sync"

	"github.com/kooshapari/BifrostGo/api/graphql/gen"
	"github.com/kooshapari/BifrostGo/api/graphql/model"
	"github.com/kooshapari/BifrostGo/db"
)

// Store interfaces for resolver dependencies

// ModelStore provides model CRUD operations
type ModelStore interface {
	ListModels(ctx context.Context, filter ModelFilter) ([]*model.Model, int, error)
	GetModel(ctx context.Context, id string) (*model.Model, error)
	UpdateModelStatus(ctx context.Context, id string, available bool) (*model.Model, error)
}

// PolicyStore provides policy CRUD operations
type PolicyStore interface {
	ListPolicies(ctx context.Context, filter PolicyFilter) ([]*model.Policy, error)
	GetPolicy(ctx context.Context, id string) (*model.Policy, error)
	CreatePolicy(ctx context.Context, input model.PolicyInput) (*model.Policy, error)
	UpdatePolicy(ctx context.Context, id string, input model.PolicyInput) (*model.Policy, error)
	ActivatePolicy(ctx context.Context, id string) (*model.Policy, error)
	DeactivatePolicy(ctx context.Context, id string) (*model.Policy, error)
}

// BenchmarkStore provides benchmark CRUD operations
type BenchmarkStore interface {
	ListBenchmarks(ctx context.Context, filter BenchmarkFilter) ([]*model.Benchmark, int, error)
	GetBenchmark(ctx context.Context, id string) (*model.Benchmark, error)
	CreateBenchmark(ctx context.Context, input model.BenchmarkInput) (*model.Benchmark, error)
}

// UsageStore provides usage analytics
type UsageStore interface {
	GetUsageReport(ctx context.Context, filter UsageFilter) (*model.UsageReport, error)
}

// RoutingStore provides routing history
type RoutingStore interface {
	GetRoutingHistory(ctx context.Context, filter RoutingFilter) ([]*model.RoutingHistory, int, error)
}

// ProviderStore provides provider operations
type ProviderStore interface {
	ListProviders(ctx context.Context) ([]*model.Provider, error)
	GetProvider(ctx context.Context, id string) (*model.Provider, error)
	RefreshToken(ctx context.Context, providerID, accountID string) (*model.Account, error)
}

// Filter types used by resolvers

// ModelFilter for querying models
type ModelFilter struct {
	Limit        int
	Provider     *string
	Capabilities []model.Capability
	Available    *bool
}

// PolicyFilter for querying policies
type PolicyFilter struct {
	Type   *model.PolicyType
	Active *bool
}

// BenchmarkFilter for querying benchmarks
type BenchmarkFilter struct {
	Limit  int
	Models []string
}

// UsageFilter for querying usage
type UsageFilter struct {
	Timeframe model.Timeframe
	GroupBy   []model.GroupByField
	Filters   *model.UsageFilters
}

// RoutingFilter for querying routing history
type RoutingFilter struct {
	Limit     int
	SessionID *string
	UserID    *string
}

// Resolver is the root resolver that provides access to all sub-resolvers.
type Resolver struct {
	db         *db.DB
	logger     *slog.Logger
	models     ModelStore
	policies   PolicyStore
	benchmarks BenchmarkStore
	usage      UsageStore
	routing    RoutingStore
	providers  ProviderStore

	// Subscription management
	mu               sync.RWMutex
	healthSubs       map[string]chan *model.ProviderHealthEvent
	availabilitySubs map[string]chan *model.ModelAvailabilityEvent
	routingSubs      map[string]chan *model.RoutingEvent
	usageSubs        map[string]chan *model.UsageUpdate
}

// ResolverOption configures a Resolver.
type ResolverOption func(*Resolver)

// NewResolver creates a new root resolver.
func NewResolver(database *db.DB, opts ...ResolverOption) *Resolver {
	r := &Resolver{
		db:               database,
		logger:           slog.Default(),
		healthSubs:       make(map[string]chan *model.ProviderHealthEvent),
		availabilitySubs: make(map[string]chan *model.ModelAvailabilityEvent),
		routingSubs:      make(map[string]chan *model.RoutingEvent),
		usageSubs:        make(map[string]chan *model.UsageUpdate),
	}
	for _, opt := range opts {
		opt(r)
	}
	return r
}

type queryResolver struct{ *Resolver }
type mutationResolver struct{ *Resolver }

// Query returns the query resolver.
func (r *Resolver) Query() gen.QueryResolver {
	return &queryResolver{r}
}

// Subscription returns the subscription resolver.
func (r *Resolver) Subscription() gen.SubscriptionResolver {
	return &subscriptionResolver{r}
}

// Mutation returns the mutation resolver.
func (r *Resolver) Mutation() gen.MutationResolver {
	return &mutationResolver{r}
}
