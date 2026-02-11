# Agents.md - Implementation Prompt Analysis & Documentation

This document provides a structured approach for analyzing implementation prompts, proposing multiple solutions with confidence ratings, and documenting implementations for future reference.

## Purpose

This document serves as a standardized template for:
1. **Rigorous Analysis**: Deep dive into implementation requirements with web research
2. **Multiple Solutions**: Present three distinct approaches with confidence ratings
3. **Testing Workflow**: Comprehensive testing including format, type checks, and linting
4. **Future Reference**: Clear documentation for maintainability and knowledge transfer

---

## Implementation Prompt Analysis Template

### 1. Prompt Understanding

**Original Request:**
```
[Copy the exact implementation request here]
```

**Extracted Requirements:**
- [ ] Functional Requirements
- [ ] Non-Functional Requirements (Performance, Security, etc.)
- [ ] Technical Constraints
- [ ] Integration Points
- [ ] Success Criteria

**Context Analysis:**
- **Project Type**: [e.g., WebSocket Provider, API Service, CLI Tool]
- **Technology Stack**: [e.g., Rust, WasmCloud, tokio]
- **Existing Codebase**: [Reference relevant files/modules]
- **Dependencies**: [List relevant dependencies]

### 2. Research Phase

**Web Search Queries Performed:**
1. [Query 1: Technology-specific best practices]
2. [Query 2: Similar implementations]
3. [Query 3: Security considerations]
4. [Query 4: Performance optimization]

**Key Findings:**
- **Finding 1**: [Summary with source]
- **Finding 2**: [Summary with source]
- **Finding 3**: [Summary with source]

---

## Three Solution Approaches

### Solution 1: [Descriptive Name]

**Description:**
[Detailed explanation of the approach]

**Implementation Steps:**
1. [Step 1]
2. [Step 2]
3. [Step 3]

**Pros:**
- ✅ [Advantage 1]
- ✅ [Advantage 2]
- ✅ [Advantage 3]

**Cons:**
- ❌ [Disadvantage 1]
- ❌ [Disadvantage 2]

**Technical Considerations:**
- **Complexity**: [Low/Medium/High]
- **Maintainability**: [Low/Medium/High]
- **Performance Impact**: [Minimal/Moderate/Significant]
- **Security Risk**: [Low/Medium/High]

**Confidence Rating: [X%]**

**Rationale:**
[1-2 sentences explaining why this confidence rating was assigned, including uncertainties or risks]

---

### Solution 2: [Descriptive Name]

**Description:**
[Detailed explanation of the approach]

**Implementation Steps:**
1. [Step 1]
2. [Step 2]
3. [Step 3]

**Pros:**
- ✅ [Advantage 1]
- ✅ [Advantage 2]
- ✅ [Advantage 3]

**Cons:**
- ❌ [Disadvantage 1]
- ❌ [Disadvantage 2]

**Technical Considerations:**
- **Complexity**: [Low/Medium/High]
- **Maintainability**: [Low/Medium/High]
- **Performance Impact**: [Minimal/Moderate/Significant]
- **Security Risk**: [Low/Medium/High]

**Confidence Rating: [Y%]**

**Rationale:**
[1-2 sentences explaining why this confidence rating was assigned, including uncertainties or risks]

---

### Solution 3: [Descriptive Name]

**Description:**
[Detailed explanation of the approach]

**Implementation Steps:**
1. [Step 1]
2. [Step 2]
3. [Step 3]

**Pros:**
- ✅ [Advantage 1]
- ✅ [Advantage 2]
- ✅ [Advantage 3]

**Cons:**
- ❌ [Disadvantage 1]
- ❌ [Disadvantage 2]

**Technical Considerations:**
- **Complexity**: [Low/Medium/High]
- **Maintainability**: [Low/Medium/High]
- **Performance Impact**: [Minimal/Moderate/Significant]
- **Security Risk**: [Low/Medium/High]

**Confidence Rating: [Z%]**

**Rationale:**
[1-2 sentences explaining why this confidence rating was assigned, including uncertainties or risks]

---

## Solution Comparison Matrix

| Criteria | Solution 1 | Solution 2 | Solution 3 |
|----------|-----------|-----------|-----------|
| **Confidence** | X% | Y% | Z% |
| **Complexity** | [Rating] | [Rating] | [Rating] |
| **Time to Implement** | [Estimate] | [Estimate] | [Estimate] |
| **Maintainability** | [Rating] | [Rating] | [Rating] |
| **Performance** | [Rating] | [Rating] | [Rating] |
| **Security** | [Rating] | [Rating] | [Rating] |
| **Best For** | [Use case] | [Use case] | [Use case] |

**Recommended Solution: [Solution X]**

**Justification:**
[2-3 sentences explaining why this solution is recommended based on the specific context]

---

## Implementation Checklist

### Pre-Implementation
- [ ] Review existing codebase and architecture
- [ ] Identify affected files and modules
- [ ] Check for dependencies that need updating
- [ ] Review security advisories for dependencies
- [ ] Create feature branch

### Implementation Phase
- [ ] Implement core functionality
- [ ] Add error handling
- [ ] Add logging/tracing
- [ ] Update configuration if needed
- [ ] Ensure backward compatibility

### Testing Workflow

#### 1. Code Formatting
```bash
# Rust projects
cargo fmt --all -- --check

# Fix formatting issues
cargo fmt --all
```
- [ ] Run formatter check
- [ ] Fix any formatting issues
- [ ] Verify changes with git diff

#### 2. Type Checking
```bash
# Rust projects
cargo check --all-targets
cargo check --release --all-targets

# For WebAssembly components
cargo check --target wasm32-wasip2 --manifest-path component/Cargo.toml
```
- [ ] Run type checker on provider
- [ ] Run type checker on component (if applicable)
- [ ] Fix any type errors
- [ ] Verify no warnings

#### 3. Linting (Clippy for Rust)
```bash
# Strict mode - treat warnings as errors
cargo clippy --release -- -D warnings

# For WebAssembly components
cargo clippy --release --target wasm32-wasip2 --manifest-path component/Cargo.toml -- -D warnings
```
- [ ] Run clippy on provider code
- [ ] Run clippy on component code (if applicable)
- [ ] Address all warnings and errors
- [ ] Review clippy suggestions for code quality

#### 4. Build Verification
```bash
# Build provider
wash build

# Build component (if applicable)
wash build -p ./component
```
- [ ] Build provider successfully
- [ ] Build component successfully (if applicable)
- [ ] Verify artifacts are generated
- [ ] Check build output for warnings

#### 5. Unit Tests
```bash
# Run unit tests
cargo test

# Run specific test
cargo test [test_name]

# Run with output
cargo test -- --nocapture
```
- [ ] Write/update unit tests for new functionality
- [ ] Run all unit tests
- [ ] Ensure 100% pass rate
- [ ] Verify test coverage for critical paths

#### 6. Integration Tests
```bash
# Run integration test script
./tests/run_integration_test.sh

# Or manual testing steps from TESTING.md
```
- [ ] Run integration tests
- [ ] Verify end-to-end functionality
- [ ] Test error scenarios
- [ ] Validate reconnection logic (if applicable)
- [ ] Check resource cleanup

#### 7. Security Scanning
```bash
# Check for security vulnerabilities
cargo audit

# Update vulnerable dependencies if needed
cargo update
```
- [ ] Run security audit
- [ ] Address any vulnerabilities
- [ ] Document accepted risks (if any)
- [ ] Verify no secrets in code

### Post-Implementation
- [ ] Update README.md if user-facing changes
- [ ] Update TESTING.md if new test procedures
- [ ] Update configuration documentation
- [ ] Add/update inline code comments
- [ ] Update this Agents.md with implementation details
- [ ] Create/update examples
- [ ] Review CI/CD pipeline passes
- [ ] Request code review

---

## Implementation Documentation

### [Feature/Change Name] - [Date]

**Problem Statement:**
[Describe what problem was being solved]

**Solution Implemented:**
[Which solution was chosen and why]

**Files Modified:**
- `[file1.rs]`: [Brief description of changes]
- `[file2.rs]`: [Brief description of changes]
- `[config.yml]`: [Brief description of changes]

**Key Code Changes:**
```rust
// Example of key implementation detail
[Code snippet with explanation]
```

**Configuration Changes:**
```yaml
# New configuration options
[Config snippet]
```

**Testing Performed:**
- ✅ Format check: Passed
- ✅ Type check: Passed
- ✅ Clippy: Passed with 0 warnings
- ✅ Unit tests: [X/X] passed
- ✅ Integration tests: Passed
- ✅ Security audit: No vulnerabilities

**Performance Impact:**
[Describe any performance implications]

**Breaking Changes:**
[List any breaking changes or "None"]

**Migration Guide:**
[If breaking changes, provide migration steps]

**Lessons Learned:**
- [Lesson 1]
- [Lesson 2]
- [Lesson 3]

**Future Improvements:**
- [Potential enhancement 1]
- [Potential enhancement 2]

**References:**
- [Link to related issue/PR]
- [Link to relevant documentation]
- [Link to external resources]

---

## Quick Reference

### Common Testing Commands

```bash
# Full test suite
cargo fmt --all -- --check && \
cargo clippy --release -- -D warnings && \
cargo clippy --release --target wasm32-wasip2 --manifest-path component/Cargo.toml -- -D warnings && \
wash build && \
wash build -p ./component && \
./tests/run_integration_test.sh

# Quick validation
cargo fmt --all -- --check && cargo clippy --release -- -D warnings && cargo test

# Build only
wash build && wash build -p ./component

# Security check
cargo audit
```

### Project-Specific Context

**Project**: WasmCloud WebSocket Provider
**Language**: Rust 2021 Edition
**Key Dependencies**: 
- `wasmcloud-provider-sdk` v0.13.0
- `tokio-tungstenite` v0.24
- `wit-bindgen-wrpc` v0.9.0

**Architecture**:
```
WebSocket Server → WebSocket Provider (Rust) → wRPC → wasmCloud Component (Wasm)
```

**CI Pipeline**: GitHub Actions
- Check & Lint → Build → Integration Test

---

## Template Usage Guide

### For New Implementation Requests:

1. **Copy this template** to a new section at the bottom of this file
2. **Fill in the "Prompt Understanding"** section with the specific request
3. **Conduct web research** and document findings
4. **Generate three distinct solutions** with pros/cons and confidence ratings
5. **Create comparison matrix** to evaluate trade-offs
6. **Select and implement** the recommended solution
7. **Follow the testing checklist** meticulously
8. **Document the implementation** in the "Implementation Documentation" section
9. **Update the Quick Reference** if new patterns emerge

### Confidence Rating Guidelines:

- **90-100%**: High confidence, well-understood problem with proven solutions
- **70-89%**: Good confidence, some uncertainties but manageable risks
- **50-69%**: Moderate confidence, significant unknowns or complexity
- **30-49%**: Low confidence, high risk or unproven approach
- **0-29%**: Very low confidence, experimental or highly uncertain

---

## Maintenance Notes

**Last Updated**: [Date]
**Maintained By**: Development Team
**Review Frequency**: After each major implementation

**Document Version**: 1.0.0

---

## Example Implementation: Add Connection Timeout Feature

### 1. Prompt Understanding

**Original Request:**
```
Add a configurable connection timeout feature to the WebSocket provider to prevent 
hanging connections. The timeout should be configurable via link configuration and 
have a sensible default value.
```

**Extracted Requirements:**
- [x] Functional Requirements
  - Add connection timeout configuration option
  - Implement timeout logic during WebSocket connection establishment
  - Use configurable timeout value from link configuration
  - Apply default timeout if not configured
- [x] Non-Functional Requirements (Performance, Security, etc.)
  - Minimal performance overhead
  - Should not affect existing connections
  - Graceful timeout handling without panics
- [x] Technical Constraints
  - Must work with tokio-tungstenite
  - Configuration through existing LinkConfig system
  - Backward compatible with existing deployments
- [x] Integration Points
  - Link configuration parsing
  - WebSocket connection logic in provider
  - Error handling and logging
- [x] Success Criteria
  - Connection times out after configured duration
  - Clear error message on timeout
  - No impact on successful connections

**Context Analysis:**
- **Project Type**: WebSocket Provider for WasmCloud
- **Technology Stack**: Rust, tokio, tokio-tungstenite, wasmcloud-provider-sdk
- **Existing Codebase**: `src/provider.rs` contains connection logic
- **Dependencies**: tokio (already included with timeout features)

### 2. Research Phase

**Web Search Queries Performed:**
1. "tokio tungstenite connection timeout best practices"
2. "Rust WebSocket client timeout implementation"
3. "tokio timeout patterns for network connections"
4. "WebSocket connection timeout security considerations"

**Key Findings:**
- **Finding 1**: tokio provides `timeout()` function that wraps futures with a duration - this is the idiomatic approach for timeouts in async Rust
- **Finding 2**: Connection timeouts typically range from 5-30 seconds; 10 seconds is a common default for WebSocket connections
- **Finding 3**: Timeout errors should be distinct from other connection errors for better debugging and monitoring

---

## Three Solution Approaches

### Solution 1: Use tokio::time::timeout with Duration from config

**Description:**
Use `tokio::time::timeout()` to wrap the WebSocket connection future. Parse timeout value from link configuration as milliseconds, convert to Duration, and apply during connection establishment.

**Implementation Steps:**
1. Add `connection_timeout_ms` field to link configuration with default value of 10000ms
2. Parse config value in `handle_link_config()` function
3. Wrap `connect_async()` call with `tokio::time::timeout(duration, connect_async(url))`
4. Handle timeout error specifically and log appropriately

**Pros:**
- ✅ Idiomatic Rust/tokio approach
- ✅ Simple implementation (~20 lines of code)
- ✅ No additional dependencies needed
- ✅ Well-tested tokio timeout mechanism
- ✅ Easy to test and debug

**Cons:**
- ❌ Requires adding another configuration field
- ❌ Timeout applies to entire connection handshake

**Technical Considerations:**
- **Complexity**: Low
- **Maintainability**: High - uses standard library features
- **Performance Impact**: Minimal (tokio timer overhead is negligible)
- **Security Risk**: Low - prevents hanging connections

**Confidence Rating: 95%**

**Rationale:**
This is the standard Rust/tokio pattern for implementing timeouts. High confidence due to proven approach, excellent documentation, and wide usage in production systems. The 5% uncertainty comes from potential edge cases with TLS handshakes that might need longer timeouts.

---

### Solution 2: Implement custom timeout with select! macro

**Description:**
Use `tokio::select!` to race the connection future against a sleep future. Manually manage the timeout logic and cancellation.

**Implementation Steps:**
1. Add `connection_timeout_ms` configuration field
2. Create a sleep future with the timeout duration
3. Use `select!` to race between `connect_async()` and `sleep()`
4. Handle timeout branch to return appropriate error

**Pros:**
- ✅ More control over timeout behavior
- ✅ Can add custom logic in timeout branch
- ✅ Could implement retry logic within same select block

**Cons:**
- ❌ More code to write and maintain (~40 lines)
- ❌ More complex than necessary for simple timeout
- ❌ Easier to introduce bugs with manual cancellation
- ❌ Less idiomatic than using timeout() directly

**Technical Considerations:**
- **Complexity**: Medium
- **Maintainability**: Medium - custom logic requires more maintenance
- **Performance Impact**: Minimal (same underlying mechanism as Solution 1)
- **Security Risk**: Low

**Confidence Rating: 75%**

**Rationale:**
This approach works but is unnecessarily complex for a simple timeout. Lower confidence because it introduces more potential for bugs and doesn't provide significant benefits over Solution 1. Would only choose this if we needed very custom timeout behavior.

---

### Solution 3: Use TCP-level timeout with socket options

**Description:**
Set socket-level timeout options (`SO_RCVTIMEO`, `SO_SNDTIMEO`) before establishing WebSocket connection. This applies timeout at the TCP layer.

**Implementation Steps:**
1. Create TCP socket manually with timeout options
2. Configure socket with desired timeout values
3. Pass configured socket to `client_async_tls_with_config()`
4. Handle socket timeout errors

**Pros:**
- ✅ Timeout applies at lower network layer
- ✅ Catches network-level hangs earlier

**Cons:**
- ❌ Significantly more complex (~60+ lines of code)
- ❌ Platform-specific behavior differences
- ❌ Requires manual socket creation and TLS configuration
- ❌ Less portable across platforms
- ❌ Harder to test
- ❌ Conflicts with tokio's async model

**Technical Considerations:**
- **Complexity**: High
- **Maintainability**: Low - platform-specific code
- **Performance Impact**: Minimal
- **Security Risk**: Medium - manual socket/TLS handling increases risk

**Confidence Rating: 45%**

**Rationale:**
This approach is overly complex and moves away from tokio-tungstenite's abstractions. Low confidence due to complexity, portability concerns, and difficulty in testing. The benefits don't justify the additional complexity and maintenance burden.

---

## Solution Comparison Matrix

| Criteria | Solution 1 | Solution 2 | Solution 3 |
|----------|-----------|-----------|-----------|
| **Confidence** | 95% | 75% | 45% |
| **Complexity** | Low | Medium | High |
| **Time to Implement** | 30 min | 1-2 hours | 3-4 hours |
| **Maintainability** | High | Medium | Low |
| **Performance** | Excellent | Excellent | Excellent |
| **Security** | Low Risk | Low Risk | Medium Risk |
| **Best For** | Standard timeout needs | Custom retry logic | Low-level control needed |

**Recommended Solution: Solution 1 (tokio::time::timeout)**

**Justification:**
Solution 1 is the clear winner with 95% confidence. It's idiomatic Rust, simple to implement and maintain, well-tested, and provides exactly the functionality needed. The tokio timeout mechanism is production-proven and requires minimal code changes. Solutions 2 and 3 add unnecessary complexity without providing meaningful benefits for this use case.

---

## Implementation Documentation

### Connection Timeout Feature - 2026-02-11

**Problem Statement:**
The WebSocket provider could hang indefinitely when attempting to connect to unresponsive servers, leading to resource exhaustion and poor user experience. A configurable timeout was needed to prevent this issue.

**Solution Implemented:**
Solution 1 - tokio::time::timeout wrapper around connection establishment

**Files Modified:**
- `src/provider.rs`: Added connection timeout configuration parsing and applied timeout to connect_async() call
- `README.md`: Updated configuration table to include `connection_timeout_ms` option
- `tests/test_timeout.rs`: Added integration test for timeout behavior (if implemented)

**Key Code Changes:**
```rust
// In LinkConfig struct
pub struct LinkConfig {
    pub websocket_url: String,
    pub max_reconnect_attempts: u32,
    pub initial_reconnect_delay_ms: u64,
    pub max_reconnect_delay_ms: u64,
    pub max_message_size: usize,
    pub connection_timeout_ms: u64, // NEW: Default 10000ms
}

// In connection establishment
async fn connect_websocket(&self, url: &str) -> Result<WebSocketStream<...>> {
    let timeout_duration = Duration::from_millis(self.config.connection_timeout_ms);
    
    match tokio::time::timeout(timeout_duration, connect_async(url)).await {
        Ok(Ok((ws_stream, response))) => {
            tracing::info!("WebSocket connection established: {}", response.status());
            Ok(ws_stream)
        }
        Ok(Err(e)) => {
            tracing::error!("WebSocket connection error: {}", e);
            Err(e.into())
        }
        Err(_) => {
            let err_msg = format!(
                "Connection timeout after {}ms to {}",
                self.config.connection_timeout_ms, url
            );
            tracing::error!("{}", err_msg);
            Err(anyhow::anyhow!(err_msg))
        }
    }
}
```

**Configuration Changes:**
```yaml
# In wadm.yaml - example usage
properties:
  websocket_config:
    websocket_url: "ws://example.com:8080"
    connection_timeout_ms: "15000"  # 15 second timeout
```

**Testing Performed:**
- ✅ Format check: Passed
- ✅ Type check: Passed  
- ✅ Clippy: Passed with 0 warnings
- ✅ Unit tests: 12/12 passed
- ✅ Integration tests: Passed (tested with unresponsive server)
- ✅ Security audit: No vulnerabilities

**Performance Impact:**
Negligible - tokio timer overhead is less than 1ms. No impact on successful connections.

**Breaking Changes:**
None - new configuration field has a sensible default (10000ms)

**Migration Guide:**
No migration needed. Existing deployments will use the 10-second default timeout automatically.

**Lessons Learned:**
- Always prefer tokio's built-in timeout mechanisms over custom implementations
- Configuration defaults should match industry standards (10s for WebSocket is standard)
- Clear error messages on timeout help with debugging connectivity issues
- Testing timeout behavior requires careful test infrastructure (mock servers)

**Future Improvements:**
- Consider separate timeouts for TLS handshake vs WebSocket handshake
- Add metrics/monitoring for timeout frequency
- Consider adaptive timeout based on historical connection times

**References:**
- [Tokio timeout documentation](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html)
- [RFC 6455 WebSocket Protocol](https://tools.ietf.org/html/rfc6455)
- Issue #N (replace with actual issue number)
- PR #N (replace with actual PR number)
