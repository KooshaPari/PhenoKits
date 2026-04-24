// Package metrics provides Prometheus metrics collection for bifrost-extensions.
package metrics

import (
	"context"
	"fmt"
	"net/http"
	"sync"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

// Metrics holds all Prometheus metrics for bifrost-extensions
type Metrics struct {
	// Request metrics
	RequestsTotal    *prometheus.CounterVec
	RequestDuration  *prometheus.HistogramVec
	RequestsInFlight prometheus.Gauge

	// Token metrics
	TokensInTotal  prometheus.Counter
	TokensOutTotal prometheus.Counter
	TokensTotal    prometheus.Counter

	// Cost metrics
	CostUSDTotal prometheus.Counter

	// Error metrics
	ErrorsTotal *prometheus.CounterVec

	// Session metrics
	ActiveSessions   prometheus.Gauge
	SessionStarts    prometheus.Counter
	SessionTransfers prometheus.Counter

	// Model metrics
	ModelRequestsTotal *prometheus.CounterVec
	ModelTokensTotal    *prometheus.CounterVec

	// Rate limit metrics
	RateLimitHits prometheus.Counter
	RateLimitHeadroom prometheus.GaugeVec

	// Agent metrics
	AgentMessagesTotal prometheus.Counter
	AgentEventsTotal   prometheus.Counter

	mu sync.RWMutex
}

// New creates a new Metrics instance
func New(namespace string) *Metrics {
	m := &Metrics{}

	// Request metrics
	m.RequestsTotal = promauto.NewCounterVec(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "requests_total",
			Help:      "Total number of requests",
		},
		[]string{"method", "endpoint", "status"},
	)

	m.RequestDuration = promauto.NewHistogramVec(
		prometheus.HistogramOpts{
			Namespace: namespace,
			Name:      "request_duration_seconds",
			Help:      "Request duration in seconds",
			Buckets:   prometheus.DefBuckets,
		},
		[]string{"method", "endpoint"},
	)

	m.RequestsInFlight = promauto.NewGauge(
		prometheus.GaugeOpts{
			Namespace: namespace,
			Name:      "requests_in_flight",
			Help:      "Number of requests currently being processed",
		},
	)

	// Token metrics
	m.TokensInTotal = promauto.NewCounter(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "tokens_in_total",
			Help:      "Total number of input tokens",
		},
	)

	m.TokensOutTotal = promauto.NewCounter(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "tokens_out_total",
			Help:      "Total number of output tokens",
		},
	)

	m.TokensTotal = promauto.NewCounter(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "tokens_total",
			Help:      "Total number of tokens (in + out)",
		},
	)

	// Cost metrics
	m.CostUSDTotal = promauto.NewCounter(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "cost_usd_total",
			Help:      "Total cost in USD",
		},
	)

	// Error metrics
	m.ErrorsTotal = promauto.NewCounterVec(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "errors_total",
			Help:      "Total number of errors",
		},
		[]string{"type", "endpoint"},
	)

	// Session metrics
	m.ActiveSessions = promauto.NewGauge(
		prometheus.GaugeOpts{
			Namespace: namespace,
			Name:      "active_sessions",
			Help:      "Number of active sessions",
		},
	)

	m.SessionStarts = promauto.NewCounter(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "session_starts_total",
			Help:      "Total number of session starts",
		},
	)

	m.SessionTransfers = promauto.NewCounter(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "session_transfers_total",
			Help:      "Total number of session transfers",
		},
	)

	// Model metrics
	m.ModelRequestsTotal = promauto.NewCounterVec(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "model_requests_total",
			Help:      "Total number of requests per model",
		},
		[]string{"model"},
	)

	m.ModelTokensTotal = promauto.NewCounterVec(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "model_tokens_total",
			Help:      "Total tokens per model",
		},
		[]string{"model", "direction"}, // direction: "in" or "out"
	)

	// Rate limit metrics
	m.RateLimitHits = promauto.NewCounter(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "rate_limit_hits_total",
			Help:      "Total number of rate limit hits",
		},
	)

	m.RateLimitHeadroom = promauto.NewGaugeVec(
		prometheus.GaugeOpts{
			Namespace: namespace,
			Name:      "rate_limit_headroom",
			Help:      "Current headroom for rate limits (0-1)",
		},
		[]string{"limit_type", "account_id"},
	)

	// Agent metrics
	m.AgentMessagesTotal = promauto.NewCounter(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "agent_messages_total",
			Help:      "Total number of agent messages",
		},
	)

	m.AgentEventsTotal = promauto.NewCounter(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "agent_events_total",
			Help:      "Total number of agent events",
		},
	)

	return m
}

// RecordRequest records a request
func (m *Metrics) RecordRequest(method, endpoint string, status int, duration time.Duration) {
	m.RequestsTotal.WithLabelValues(method, endpoint, fmt.Sprintf("%d", status)).Inc()
	m.RequestDuration.WithLabelValues(method, endpoint).Observe(duration.Seconds())
}

// RecordTokens records token usage
func (m *Metrics) RecordTokens(tokensIn, tokensOut int) {
	m.TokensInTotal.Add(float64(tokensIn))
	m.TokensOutTotal.Add(float64(tokensOut))
	m.TokensTotal.Add(float64(tokensIn + tokensOut))
}

// RecordCost records cost in USD
func (m *Metrics) RecordCost(costUSD float64) {
	m.CostUSDTotal.Add(costUSD)
}

// RecordError records an error
func (m *Metrics) RecordError(errorType, endpoint string) {
	m.ErrorsTotal.WithLabelValues(errorType, endpoint).Inc()
}

// RecordModelRequest records a request for a specific model
func (m *Metrics) RecordModelRequest(model string, tokensIn, tokensOut int) {
	m.ModelRequestsTotal.WithLabelValues(model).Inc()
	m.ModelTokensTotal.WithLabelValues(model, "in").Add(float64(tokensIn))
	m.ModelTokensTotal.WithLabelValues(model, "out").Add(float64(tokensOut))
}

// RecordRateLimitHit records a rate limit hit
func (m *Metrics) RecordRateLimitHit() {
	m.RateLimitHits.Inc()
}

// SetRateLimitHeadroom sets the current headroom for a rate limit
func (m *Metrics) SetRateLimitHeadroom(limitType, accountID string, headroom float64) {
	m.RateLimitHeadroom.WithLabelValues(limitType, accountID).Set(headroom)
}

// SetActiveSessions sets the number of active sessions
func (m *Metrics) SetActiveSessions(count int) {
	m.ActiveSessions.Set(float64(count))
}

// IncrementSessionStarts increments the session starts counter
func (m *Metrics) IncrementSessionStarts() {
	m.SessionStarts.Inc()
}

// IncrementSessionTransfers increments the session transfers counter
func (m *Metrics) IncrementSessionTransfers() {
	m.SessionTransfers.Inc()
}

// RecordAgentMessage records an agent message
func (m *Metrics) RecordAgentMessage() {
	m.AgentMessagesTotal.Inc()
}

// RecordAgentEvent records an agent event
func (m *Metrics) RecordAgentEvent() {
	m.AgentEventsTotal.Inc()
}

// RequestMiddleware creates a middleware for recording request metrics
func (m *Metrics) RequestMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		m.RequestsInFlight.Inc()
		defer m.RequestsInFlight.Dec()

		start := time.Now()
		status := 200

		// Wrap response writer to capture status
		wrapped := &statusWriter{ResponseWriter: w, status: &status}

		next.ServeHTTP(wrapped, r)

		duration := time.Since(start)
		endpoint := r.URL.Path
		m.RecordRequest(r.Method, endpoint, status, duration)
	})
}

type statusWriter struct {
	http.ResponseWriter
	status *int
}

func (w *statusWriter) WriteHeader(status int) {
	*w.status = status
	w.ResponseWriter.WriteHeader(status)
}

// Handler returns the Prometheus metrics handler
func (m *Metrics) Handler() http.Handler {
	return promhttp.Handler()
}

// Server is a Prometheus metrics server
type Server struct {
	metrics  *Metrics
	server   *http.Server
	registry *prometheus.Registry
}

// NewServer creates a new metrics server
func NewServer(addr string, m *Metrics) *Server {
	mux := http.NewServeMux()
	mux.Handle("/metrics", promhttp.Handler())

	return &Server{
		metrics: m,
		server: &http.Server{
			Addr:    addr,
			Handler: mux,
		},
	}
}

// Start starts the metrics server
func (s *Server) Start() error {
	return s.server.ListenAndServe()
}

// Stop stops the metrics server
func (s *Server) Stop(ctx context.Context) error {
	return s.server.Shutdown(ctx)
}
