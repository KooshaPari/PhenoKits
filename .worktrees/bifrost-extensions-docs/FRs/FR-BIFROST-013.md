# FR-BIFROST-013: Chat Completion with Nil Request Handling

**Requirement:** ChatCompletion properly handles nil ChatRequest with 400 error.

**Code Location:** `providers/agentcli/provider.go`

**Repository:** bifrost-extensions

**Status:** Active

**Test Traces:** `providers/agentcli/provider_test.go::TestChatCompletionNilRequest`
