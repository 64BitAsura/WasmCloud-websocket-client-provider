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
