// Package costengine provides cost calculation and limit checking for the Bifrost API.
package costengine

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/kooshapari/bifrost-extensions/config"
)

// Engine represents the cost calculation engine
type Engine struct {
	config     *config.Config
	db         *sql.DB
	queries    interface{} // Will be *sqlc.Queries when available
}

// LimitType represents the type of limit being checked
type LimitType string

const (
	// LimitTokensPerMin limits tokens per minute
	LimitTokensPerMin LimitType = "tokens_per_min"
	// LimitTokensPerHour limits tokens per hour
	LimitTokensPerHour LimitType = "tokens_per_hour"
	// LimitTokensPerDay limits tokens per day
	LimitTokensPerDay LimitType = "tokens_per_day"
	// LimitRequestsPerMin limits requests per minute
	LimitRequestsPerMin LimitType = "requests_per_min"
	// LimitRequestsPerDay limits requests per day
	LimitRequestsPerDay LimitType = "requests_per_day"
	// LimitCreditsPerMonth limits credits per month
	LimitCreditsPerMonth LimitType = "credits_per_month"
)

// AccountLimit represents a limit for an account
type AccountLimit struct {
	AccountID   interface{}
	LimitType   LimitType
	LimitValue  float64
	WindowStart time.Time
	WindowEnd   time.Time
}

// UsageSnapshot represents a usage record
type UsageSnapshot struct {
	ID          int64
	AccountID   interface{}
	WindowType  string
	WindowStart time.Time
	WindowEnd   time.Time
	TokensIn    int
	TokensOut   int
	CostUSD     float64
	UpdatedAt   time.Time
}

// NewEngine creates a new cost engine
func NewEngine(cfg *config.Config, db *sql.DB) *Engine {
	return &Engine{
		config: cfg,
		db:    db,
	}
}

// CheckLimit checks if a request would exceed a limit and returns the headroom
// Headroom is a value between 0 and 1, where 0 means no room and 1 means plenty of room
func (e *Engine) CheckLimit(ctx context.Context, accountID interface{}, limit AccountLimit, tokensIn, tokensOut int) (float64, string) {
	headroom, reason := e.checkSingleLimit(ctx, accountID, limit, tokensIn, tokensOut)
	return headroom, reason
}

// checkSingleLimit checks a single limit and returns headroom
func (e *Engine) checkSingleLimit(ctx context.Context, accountID interface{}, limit AccountLimit, tokensIn, tokensOut int) (float64, string) {
	// Get current usage from database using window calculation
	now := time.Now().UTC()

	currentUsage, err := e.QueryUsage(ctx, accountID, limit.LimitType, now)
	if err != nil {
		// On error, be conservative and deny
		return 0, "failed to query usage: " + err.Error()
	}

	// Calculate predicted usage after this request
	var requestUsage float64
	switch limit.LimitType {
	case LimitTokensPerMin, LimitTokensPerHour, LimitTokensPerDay:
		requestUsage = float64(tokensIn + tokensOut)
	case LimitRequestsPerMin, LimitRequestsPerDay:
		requestUsage = 1
	case LimitCreditsPerMonth:
		requestUsage = float64(tokensIn+tokensOut) / 1000 // rough estimate
	}

	predictedUsage := currentUsage + requestUsage
	headroom := 1.0 - (predictedUsage / limit.LimitValue)

	if headroom < 0 {
		headroom = 0
	}

	reason := ""
	if headroom < e.config.HardQuotaThreshold {
		reason = string(limit.LimitType) + " limit exceeded"
	}

	return headroom, reason
}

// RecordUsage records token usage for an endpoint
func (e *Engine) RecordUsage(ctx context.Context, endpointID interface{}, tokensIn, tokensOut int, costUSD float64) error {
	return e.UpsertUsageSnapshot(ctx, endpointID, tokensIn, tokensOut, costUSD)
}

// QueryUsage queries current usage for an account and limit type
func (e *Engine) QueryUsage(ctx context.Context, accountID interface{}, limitType LimitType, windowStart time.Time) (float64, error) {
	if e.db == nil {
		// No database configured, return 0
		return 0, nil
	}

	// Calculate window based on limit type
	now := time.Now().UTC()

	switch limitType {
	case LimitTokensPerMin:
		windowStart = now.Add(-1 * time.Minute)
	case LimitTokensPerHour:
		windowStart = now.Truncate(time.Hour)
	case LimitTokensPerDay:
		windowStart = time.Date(now.Year(), now.Month(), now.Day(), 0, 0, 0, 0, time.UTC)
	case LimitRequestsPerMin:
		windowStart = now.Add(-1 * time.Minute)
	case LimitRequestsPerDay:
		windowStart = time.Date(now.Year(), now.Month(), now.Day(), 0, 0, 0, 0, time.UTC)
	case LimitCreditsPerMonth:
		windowStart = time.Date(now.Year(), now.Month(), 1, 0, 0, 0, 0, time.UTC)
	default:
		windowStart = now.Add(-1 * time.Hour)
	}

	windowEnd := windowStart.Add(time.Hour)
	if limitType == LimitTokensPerMin || limitType == LimitRequestsPerMin {
		windowEnd = now
	} else if limitType == LimitTokensPerDay || limitType == LimitRequestsPerDay {
		windowEnd = windowStart.Add(24 * time.Hour)
	} else if limitType == LimitCreditsPerMonth {
		windowEnd = windowStart.AddDate(0, 1, 0)
	}

	// Query usage from database
	query := `
		SELECT COALESCE(SUM(tokens_in + tokens_out), 0) as total_usage
		FROM usage_snapshots
		WHERE account_id = ?
		AND window_start >= ?
		AND window_end <= ?
		AND window_type = ?
	`

	// Map limit type to window type
	windowType := mapLimitTypeToWindowType(limitType)

	var totalUsage float64
	err := e.db.QueryRowContext(ctx, query, accountID, windowStart, windowEnd, windowType).Scan(&totalUsage)
	if err != nil {
		return 0, err
	}

	return totalUsage, nil
}

// UpsertUsageSnapshot upserts a usage snapshot record
func (e *Engine) UpsertUsageSnapshot(ctx context.Context, accountID interface{}, tokensIn, tokensOut int, costUSD float64) error {
	if e.db == nil {
		// No database configured, skip
		return nil
	}

	now := time.Now().UTC()

	// Insert/update for all window types (minute, hour, day, month)
	windowTypes := []struct {
		Type        string
		WindowStart time.Time
		WindowEnd   time.Time
	}{
		{"minute", now.Truncate(time.Minute), now.Truncate(time.Minute).Add(time.Minute)},
		{"hour", now.Truncate(time.Hour), now.Truncate(time.Hour).Add(time.Hour)},
		{"day", time.Date(now.Year(), now.Month(), now.Day(), 0, 0, 0, 0, time.UTC), time.Date(now.Year(), now.Month(), now.Day(), 0, 0, 0, 0, time.UTC).Add(24 * time.Hour)},
		{"month", time.Date(now.Year(), now.Month(), 1, 0, 0, 0, 0, time.UTC), time.Date(now.Year(), now.Month(), 1, 0, 0, 0, 0, time.UTC).AddDate(0, 1, 0)},
	}

	for _, wt := range windowTypes {
		query := `
			INSERT INTO usage_snapshots (account_id, window_type, window_start, window_end, tokens_in, tokens_out, cost_usd, updated_at)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?)
			ON CONFLICT(account_id, window_type, window_start)
			DO UPDATE SET
				tokens_in = usage_snapshots.tokens_in + EXCLUDED.tokens_in,
				tokens_out = usage_snapshots.tokens_out + EXCLUDED.tokens_out,
				cost_usd = usage_snapshots.cost_usd + EXCLUDED.cost_usd,
				updated_at = EXCLUDED.updated_at
		`

		_, err := e.db.ExecContext(ctx, query, accountID, wt.Type, wt.WindowStart, wt.WindowEnd, tokensIn, tokensOut, costUSD, now)
		if err != nil {
			return err
		}
	}

	return nil
}

// mapLimitTypeToWindowType maps a limit type to a window type
func mapLimitTypeToWindowType(limitType LimitType) string {
	switch limitType {
	case LimitTokensPerMin, LimitRequestsPerMin:
		return "minute"
	case LimitTokensPerHour:
		return "hour"
	case LimitTokensPerDay, LimitRequestsPerDay:
		return "day"
	case LimitCreditsPerMonth:
		return "month"
	default:
		return "hour"
	}
}

// GetAccountLimits returns the limits for an account
func (e *Engine) GetAccountLimits(accountID interface{}) []AccountLimit {
	// Default limits from config
	var limits []AccountLimit

	if e.config != nil && e.config.Limits != nil {
		limits = append(limits, AccountLimit{
			AccountID:  accountID,
			LimitType:   LimitTokensPerMin,
			LimitValue:  float64(e.config.Limits.TokensPerMinute),
			WindowStart: time.Now().Add(-1 * time.Minute),
			WindowEnd:   time.Now(),
		})
		limits = append(limits, AccountLimit{
			AccountID:  accountID,
			LimitType:   LimitTokensPerHour,
			LimitValue:  float64(e.config.Limits.TokensPerHour),
			WindowStart: time.Now().Truncate(time.Hour),
			WindowEnd:   time.Now().Truncate(time.Hour).Add(time.Hour),
		})
		limits = append(limits, AccountLimit{
			AccountID:  accountID,
			LimitType:   LimitTokensPerDay,
			LimitValue:  float64(e.config.Limits.TokensPerDay),
			WindowStart: time.Date(time.Now().Year(), time.Now().Month(), time.Now().Day(), 0, 0, 0, 0, time.UTC),
			WindowEnd:   time.Date(time.Now().Year(), time.Now().Month(), time.Now().Day(), 0, 0, 0, 0, time.UTC).Add(24 * time.Hour),
		})
	}

	return limits
}

// CalculateCost calculates the cost for given token counts
func (e *Engine) CalculateCost(tokensIn, tokensOut int, model string) float64 {
	// Default pricing per 1K tokens
	pricing := map[string]struct {
		In  float64
		Out float64
	}{
		"gpt-4o":          {0.005, 0.015},
		"gpt-4o-mini":     {0.00015, 0.0006},
		"gpt-3.5-turbo":   {0.0005, 0.0015},
		"claude-3-opus":    {0.015, 0.075},
		"claude-3-sonnet":  {0.003, 0.015},
		"claude-3-haiku":   {0.00025, 0.00125},
		"default":          {0.001, 0.002},
	}

	p, ok := pricing[model]
	if !ok {
		p = pricing["default"]
	}

	return (float64(tokensIn) / 1000 * p.In) + (float64(tokensOut) / 1000 * p.Out)
}

// UsageReport represents a usage report
type UsageReport struct {
	AccountID       interface{}
	Period          string
	TotalTokens     int
	TotalTokensIn   int
	TotalTokensOut  int
	TotalCostUSD    float64
	RequestCount    int
	GeneratedAt     time.Time
}

// GenerateReport generates a usage report for an account
func (e *Engine) GenerateReport(ctx context.Context, accountID interface{}, period string) (*UsageReport, error) {
	report := &UsageReport{
		AccountID:   accountID,
		Period:      period,
		GeneratedAt: time.Now(),
	}

	if e.db == nil {
		return report, nil
	}

	query := `
		SELECT 
			COALESCE(SUM(tokens_in), 0),
			COALESCE(SUM(tokens_out), 0),
			COALESCE(SUM(cost_usd), 0),
			COUNT(*)
		FROM usage_snapshots
		WHERE account_id = ?
		AND window_type = ?
	`

	var windowType string
	switch period {
	case "minute":
		windowType = "minute"
	case "hour":
		windowType = "hour"
	case "day":
		windowType = "day"
	case "month":
		windowType = "month"
	default:
		windowType = "day"
	}

	err := e.db.QueryRowContext(ctx, query, accountID, windowType).Scan(
		&report.TotalTokensIn,
		&report.TotalTokensOut,
		&report.TotalCostUSD,
		&report.RequestCount,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to generate report: %w", err)
	}

	report.TotalTokens = report.TotalTokensIn + report.TotalTokensOut

	return report, nil
}
