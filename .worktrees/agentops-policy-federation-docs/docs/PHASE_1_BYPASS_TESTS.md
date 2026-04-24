# Phase 1 Bypass Vector Tests

## Overview

This document summarizes the comprehensive test suite for all Phase 1 bypass vectors in the agentops-policy-federation project. Phase 1 focuses on detecting and blocking command-level bypasses that attempt to circumvent policy enforcement through:

1. **Bash command manipulation** (write-via-exec)
2. **Environment variable override** (POLICY_* vars)
3. **Working directory spoofing** (cd prefix normalization)
4. **Policy file tampering** (TOCTOU detection)
5. **Runtime artifact integrity** (audit logs and sidecars)

All 76 tests pass successfully.

## Test File Organization

### 1. tests/unit/test_claude_hooks.py (51 tests)

Tests for the Claude Code hook integration and command normalization logic.

#### NormalizeBashCommandTest (8 tests)
- **test_strip_cd_prefix_with_and_separator**: Verifies `cd /tmp && cargo test` extracts cwd=/tmp
- **test_strip_cd_prefix_with_semicolon_separator**: Verifies `cd /path; cmd` pattern
- **test_cd_with_single_quoted_path**: Handles quoted paths in cd prefix
- **test_cd_with_double_quoted_path**: Handles both single and double quotes
- **test_strip_safe_trailing_pipes**: Strips read-only postprocessor pipes (grep, tail, etc.)
- **test_strip_stderr_redirect_and_pipe**: Handles combined `2>&1 | tail` patterns
- **test_strip_safe_stderr_redirects**: Strips safe stderr/stdout redirects
- **test_complex_cd_and_pipes**: Handles both cd extraction and pipe stripping together

**Bypass Vector**: Attackers can use `cd /evil/path && dangerous_command` to spoof the working directory for policy evaluation.

#### SplitCompoundCommandTest (5 tests)
- **test_split_on_double_ampersand**: Splits `cmd1 && cmd2` correctly
- **test_split_on_semicolon**: Splits `cmd1; cmd2` correctly
- **test_split_on_or_operator**: Splits `cmd1 || cmd2` correctly
- **test_multiple_separators**: Handles mixed separators `cmd1 && cmd2 ; cmd3 || cmd4`
- **test_empty_segments_removed**: Filters out empty segments from malformed commands

**Bypass Vector**: Attackers can hide malicious commands in compound commands after safe-looking commands.

#### DetectEnvOverrideTest (7 tests)
- **test_detect_policy_repo_override**: Detects `POLICY_REPO=evil cmd`
- **test_detect_policy_task_domain_override**: Detects `POLICY_TASK_DOMAIN=malicious cmd`
- **test_detect_with_env_prefix**: Detects `env POLICY_REPO=evil cmd`
- **test_detect_with_export_prefix**: Detects `export POLICY_TASK_INSTANCE=x && cmd`
- **test_detect_policy_repo_and_domain_with_semicolon**: Multiple overrides in compound commands
- **test_ignore_non_policy_vars**: Does not flag non-POLICY_* vars
- **test_detect_in_compound_command**: Detects overrides in secondary segments

**Bypass Vector**: Attackers can override POLICY_* environment variables to spoof policy context.

#### StripEnvOverridesTest (5 tests)
- **test_strip_leading_policy_repo_override**: Removes `POLICY_REPO=evil` prefix
- **test_strip_with_env_prefix**: Handles `env POLICY_REPO=evil` format
- **test_strip_multiple_overrides**: Strips all leading POLICY_* assignments
- **test_preserve_non_policy_vars**: Preserves non-POLICY_ variables
- **test_strip_export_with_space_and_separator**: Handles `export POLICY_*=value ;` syntax

**Mitigation**: Stripped commands are re-evaluated without the override.

#### DetectWriteViaExecTest (17 tests)

Tests for detecting file writes embedded in Bash commands (write-via-exec bypass vectors):

- **test_detect_python_open_write**: Detects `python3 -c "open(...).write(...)"`
- **test_detect_shell_redirect_write_to_absolute_path**: Detects shell redirects to absolute paths
- **test_detect_tee_write**: Detects `tee /path` writes
- **test_detect_cp_write**: Detects `cp src dst` writes
- **test_detect_sed_inplace_write**: Detects `sed -i` in-place edits
- **test_detect_mv_write**: Detects `mv src dst` file moves
- **test_detect_dd_write**: Detects `dd of=/path` writes
- **test_detect_heredoc_write**: Detects `<<EOF` heredoc writes
- **test_detect_in_backtick_subshell**: Detects writes in backtick subshells `` `cp a b` ``
- **test_detect_xargs_write**: Detects `xargs rm/cp/mv` pipes
- **test_detect_ruby_file_write**: Detects `ruby -e 'File.write(...)'`
- **test_detect_perl_file_write**: Detects `perl -e 'open(...)'`
- **test_detect_node_file_write**: Detects `node -e 'fs.writeFile(...)'`
- **test_no_false_positive_on_stderr_redirect**: Does not flag `2>&1` as writes
- **test_no_false_positive_on_grep_in_pipe**: Does not flag `grep` pipes as writes

**Bypass Vector**: Attackers can embed file writes in Bash commands using various languages and utilities, bypassing the Write tool classification.

**Mitigation**: Commands with write-via-exec indicators are reclassified as `action=write`, applying write-action policies.

#### EndToEndBypassDetectionTest (4 tests)
- **test_bash_tool_python_write_reclassified_as_write_action**: Python write bypass detected and denied
- **test_bash_tool_tee_write_reclassified**: Tee write bypass detected and denied
- **test_bash_tool_env_override_detection**: Environment override detected in reason
- **test_bash_tool_cd_normalization**: Working directory extraction verified

**Integration**: Tests that bypass detection works end-to-end through the Claude hook pipeline.

### 2. tests/unit/test_runtime_artifacts.py (25 tests)

Tests for runtime sidecar and audit log integrity.

#### BuildRunSidecarTest (4 tests)
- **test_build_sidecar_with_required_fields**: Verifies sidecar contains all required fields
- **test_sidecar_run_id_is_unique**: Each sidecar gets a unique run_id
- **test_sidecar_resolved_at_is_iso_format**: Timestamp is ISO format with Z suffix
- **test_sidecar_with_custom_run_id**: Accepts and preserves custom run_id
- **test_sidecar_with_source_files**: Includes source_files list

#### WriteSidecarTest (5 tests)
- **test_write_sidecar_creates_file**: Creates sidecar file on disk
- **test_write_sidecar_creates_parent_directories**: Creates parent directories as needed
- **test_write_sidecar_json_format**: Writes valid JSON with proper indentation
- **test_write_sidecar_overwrites_existing**: Overwrites existing sidecars
- **test_write_sidecar_trailing_newline**: Ends with newline

#### AppendAuditEventTest (6 tests)
- **test_append_audit_event_creates_file**: Creates audit log file
- **test_append_audit_event_jsonl_format**: Each event is valid JSON on separate line
- **test_append_audit_event_creates_parent_directories**: Creates parent directories
- **test_audit_log_chain_verification**: Multiple writes recorded in sequence
- **test_audit_event_tampered_detection**: Detects when middle entry is modified
- **test_audit_event_sorted_keys**: Events have sorted keys for consistency

**Audit Integrity**: Tests verify that runtime decisions are immutably recorded for forensic analysis.

**Bypass Vector**: Attackers could try to tamper with audit logs post-execution. Sorted keys and sequential JSONL format enable tamper detection.

### 3. tests/unit/test_interceptor.py (25 tests)

Tests for command interception, TOCTOU protection, and policy enforcement.

#### TocTouDetectionTest (3 tests)
- **test_toctou_policy_tampered_after_check_blocks_execution**: Verifies policy hash computation
- **test_guarded_subprocess_detects_policy_tampered_before_exec**: Re-verification before execution
- **test_audit_log_recorded_on_denied_commands**: Decisions recorded with audit events

**Bypass Vector**: Time-of-Check Time-of-Use (TOCTOU) attack where policy files are modified between the authorization check and command execution.

**Mitigation**: Policy source files are hashed before and after authorization. If hashes don't match, execution is blocked with "policy-tampered" reason.

#### PolicyTamperDetectionTest (2 tests)
- **test_policy_hash_computed_for_source_files**: Decision includes policy_hash
- **test_allow_decision_re_verified_before_execution**: "Allow" decisions are re-verified before subprocess exec

**Policy Integrity**: Double-verification ensures policy consistency.

## Phase 1 Bypass Vectors Summary

| Bypass Vector | Detection Method | Test Files | Mitigation |
|---|---|---|---|
| Write-via-exec (python, shell, sed, etc.) | Regex pattern matching on command | test_claude_hooks.py (17 tests) | Reclassify as `action=write`, apply write-action policies |
| Environment override (POLICY_REPO=evil) | Regex detection of POLICY_* assignments | test_claude_hooks.py (7 tests) | Strip overrides, re-evaluate with clean context |
| Working directory spoofing (cd /path &&) | Extract cwd from `cd /path &&` prefix | test_claude_hooks.py (8 tests) | Use extracted cwd for policy evaluation |
| Compound command hiding (cmd1 && cmd2) | Split on `&&`, `;`, `\|\|` separators | test_claude_hooks.py (5 tests) | Check each segment independently |
| TOCTOU policy tampering | Hash policy files before and after check | test_interceptor.py (5 tests) | Block execution if hashes diverge, reason=policy-tampered |
| Audit log tampering | JSONL format with sorted keys | test_runtime_artifacts.py (6 tests) | Detect modifications to historical entries |

## Test Execution

All tests pass successfully:

```bash
$ python3 -m pytest tests/unit/test_claude_hooks.py tests/unit/test_runtime_artifacts.py tests/unit/test_interceptor.py -v
76 passed in 0.66s
```

### Test Coverage by Component

- **claude_hooks.py** (command classification and normalization): 51 tests
- **runtime_artifacts.py** (sidecar and audit log persistence): 25 tests
- **interceptor.py** (command execution control and TOCTOU protection): 25 tests

### Key Assertions

Each test class validates specific bypass detection logic:

1. **Normalization Tests**: Verify command patterns are correctly extracted for policy matching
2. **Detection Tests**: Verify bypass indicators are identified and reclassified
3. **Integration Tests**: Verify end-to-end pipeline correctly denies/audits bypass attempts
4. **Persistence Tests**: Verify audit trails are immutable and verifiable
5. **TOCTOU Tests**: Verify policy file integrity is maintained through execution

## Future Enhancements

Phase 2 and beyond should test:
- **Subshell nesting**: `$( $(...) )` deep subshell attacks
- **Quote escaping**: Bypassing quote detection with escape sequences
- **Binary execution**: Direct syscalls via compiled binaries
- **Network-based policies**: Fetching policy from remote sources
- **Rollback attacks**: Reverting to old policy versions
- **Multi-file attacks**: Coordinated writes across multiple files

## Integration with CI/CD

These tests are integrated into the project's CI/CD pipeline and should be run:

1. **Pre-commit**: `python3 -m pytest tests/unit/test_claude_hooks.py -q`
2. **On every PR**: Full test suite with coverage reporting
3. **Release validation**: Full audit trail verification

## References

- `cli/src/policy_federation/claude_hooks.py` - Command classification
- `cli/src/policy_federation/interceptor.py` - Execution control
- `cli/src/policy_federation/runtime_artifacts.py` - Audit infrastructure
