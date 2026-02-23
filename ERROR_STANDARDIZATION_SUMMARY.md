# Comprehensive Error Code Standardization

## Overview
This implementation provides a comprehensive error standardization system across all StellarSpend contracts, replacing string-based errors with standardized codes, providing comprehensive documentation mapping, and enabling consistent error handling patterns.

## Key Features Implemented

### 1. Global Error Enum
- **Standardized Error Codes**: 50+ error types with unique numeric codes
- **Categorized Errors**: Organized into logical categories (1000-2199 range)
- **Severity Classification**: Critical, High, Medium, Low, Info levels
- **Recoverability Assessment**: Determines if errors are recoverable
- **Retry Strategy Mapping**: Suggested retry approaches for each error type

### 2. Error Code to Documentation Mapping
- **Comprehensive Documentation**: Detailed descriptions for all error codes
- **Common Causes**: Typical reasons for each error occurrence
- **Suggested Solutions**: Recommended fixes for each error type
- **Context Information**: Additional metadata for debugging
- **Retry Delay Suggestions**: Recommended wait times before retry

### 3. String-Based Error Replacement
- **Standardized Macros**: Consistent error handling patterns
- **Type Safety**: Compile-time error validation
- **Context Preservation**: Maintains error context information
- **Event Integration**: Automatic error event emission
- **Logging Support**: Configurable error logging

### 4. Comprehensive Error Scenario Tests
- **Complete Coverage**: Tests for all error codes and scenarios
- **Macro Testing**: Validation of all error handling macros
- **Integration Testing**: End-to-end error flow validation
- **Edge Case Coverage**: Boundary and unusual condition testing

## Core Data Structures

### StellarSpendError Enum
```rust
pub enum StellarSpendError {
    // Initialization Errors (1000-1099)
    NotInitialized = 1000,
    AlreadyInitialized = 1001,
    InvalidInitialization = 1002,
    
    // Authorization Errors (1100-1199)
    Unauthorized = 1100,
    InvalidSignature = 1101,
    InsufficientPermissions = 1102,
    AdminRequired = 1103,
    MinterRequired = 1104,
    
    // Validation Errors (1200-1299)
    InvalidInput = 1200,
    InvalidAmount = 1201,
    InvalidAddress = 1202,
    InvalidTimestamp = 1203,
    // ... and many more
}
```

### ErrorDocumentation Structure
```rust
pub struct ErrorDocumentation {
    pub code: u32,
    pub name: String,
    pub category: ErrorCategory,
    pub severity: ErrorSeverity,
    pub description: String,
    pub causes: Vec<String>,
    pub solutions: Vec<String>,
    pub recoverable: bool,
    pub retry_delay: Option<u64>,
}
```

### ErrorContext Structure
```rust
pub struct ErrorContext {
    pub error_code: u32,
    pub contract_name: String,
    pub function_name: String,
    pub parameters: Vec<String>,
    pub timestamp: u64,
    pub additional_info: Map<String, String>,
}
```

## Error Categories and Code Ranges

### 1. Initialization Errors (1000-1099)
- `NotInitialized` (1000): Contract not initialized
- `AlreadyInitialized` (1001): Contract already initialized
- `InvalidInitialization` (1002): Invalid initialization parameters

### 2. Authorization Errors (1100-1199)
- `Unauthorized` (1100): Caller not authorized
- `InvalidSignature` (1101): Invalid signature provided
- `InsufficientPermissions` (1102): Insufficient permissions
- `AdminRequired` (1103): Admin privileges required
- `MinterRequired` (1104): Minter privileges required

### 3. Validation Errors (1200-1299)
- `InvalidInput` (1200): Invalid input provided
- `InvalidAmount` (1201): Invalid amount provided
- `InvalidAddress` (1202): Invalid address provided
- `InvalidTimestamp` (1203): Invalid timestamp provided
- `InvalidParameter` (1204): Invalid parameter provided
- `InvalidConfiguration` (1205): Invalid configuration provided

### 4. State Errors (1300-1399)
- `NotFound` (1300): Resource not found
- `AlreadyExists` (1301): Resource already exists
- `InvalidState` (1302): Invalid contract state
- `NotActive` (1303): Contract not active
- `Expired` (1304): Resource expired
- `Locked` (1305): Resource locked
- `Paused` (1306): Contract paused

### 5. Balance/Amount Errors (1400-1499)
- `InsufficientBalance` (1400): Insufficient balance
- `InsufficientAllowance` (1401): Insufficient allowance
- `InsufficientLiquidity` (1402): Insufficient liquidity
- `AmountExceedsLimit` (1403): Amount exceeds limit
- `NegativeAmount` (1404): Negative amount provided
- `ZeroAmount` (1405): Zero amount provided

### 6. Limit/Cap Errors (1500-1599)
- `LimitExceeded` (1500): Operation limit exceeded
- `CapExceeded` (1501): Cap limit exceeded
- `QuotaExceeded` (1502): Quota exceeded
- `RateLimitExceeded` (1503): Rate limit exceeded
- `MaxUsersExceeded` (1504): Maximum users exceeded
- `MaxTransactionsExceeded` (1505): Maximum transactions exceeded

### 7. Arithmetic Errors (1600-1699)
- `Overflow` (1600): Arithmetic overflow
- `Underflow` (1601): Arithmetic underflow
- `DivisionByZero` (1602): Division by zero
- `InvalidCalculation` (1603): Invalid calculation

### 8. Storage Errors (1700-1799)
- `StorageError` (1700): Storage operation failed
- `CorruptedData` (1701): Data corruption
- `DataNotFound` (1702): Data not found
- `WriteFailed` (1703): Write operation failed
- `ReadFailed` (1704): Read operation failed

### 9. External Errors (1800-1899)
- `NetworkError` (1800): Network operation failed
- `ExternalCallFailed` (1801): External call failed
- `OracleUnavailable` (1802): Oracle unavailable
- `BridgeError` (1803): Bridge operation failed

### 10. Business Logic Errors (1900-1999)
- `TransactionFailed` (1900): Transaction failed
- `ConditionNotMet` (1901): Condition not met
- `DeadlineExceeded` (1902): Deadline exceeded
- `IncompatibleOperation` (1903): Incompatible operation
- `InvalidOperation` (1904): Invalid operation

### 11. Security Errors (2000-2099)
- `SecurityViolation` (2000): Security violation
- `SuspiciousActivity` (2001): Suspicious activity
- `BlacklistedAddress` (2002): Blacklisted address
- `FrozenAccount` (2003): Frozen account
- `ComplianceViolation` (2004): Compliance violation

### 12. System Errors (2100-2199)
- `SystemError` (2100): System error
- `InternalError` (2101): Internal error
- `NotImplemented` (2102): Feature not implemented
- `MaintenanceMode` (2103): Maintenance mode
- `UpgradeRequired` (2104): Upgrade required

## Standardized Error Handling Macros

### Basic Error Macro
```rust
std_error!(env, StellarSpendError::InvalidInput);
std_error!(env, StellarSpendError::Unauthorized, context);
```

### Validation Macros
```rust
validate!(env, condition, StellarSpendError::InvalidInput);
validate!(env, condition, StellarSpendError::InvalidInput, "message");
```

### Authorization Macros
```rust
require_auth!(env, caller, required_address);
require_admin!(env, caller);
```

### Amount Validation Macros
```rust
validate_amount!(env, amount);
validate_amount!(env, amount, min_value);
validate_amount!(env, amount, min_value, max_value);
```

### Address Validation Macros
```rust
validate_address!(env, address);
```

### Safe Arithmetic Macros
```rust
safe_add!(env, a, b);
safe_sub!(env, a, b);
safe_mul!(env, a, b);
safe_div!(env, a, b);
```

## Error Severity Classification

### Critical (Level 4)
- Security violations
- System errors
- Internal errors
- Data corruption

### High (Level 3)
- Authorization failures
- Insufficient balance/allowance
- Arithmetic overflow/underflow
- Storage errors

### Medium (Level 2)
- Invalid input/parameters
- Limit/cap exceeded
- Rate limiting

### Low (Level 1)
- Resource not found
- Expired resources
- Inactive/paused state

### Info (Level 0)
- Informational messages
- Non-critical warnings

## Retry Strategy Mapping

### No Retry
- Permanent errors (unauthorized, security violations)
- Insufficient balance
- Invalid configuration

### Immediate Retry
- Transient network errors
- Oracle availability issues

### Fixed Delay
- Maintenance mode
- System upgrades

### Exponential Backoff
- Rate limiting
- Temporary system issues
- Default for unknown errors

## Utility Functions

### ContractUtils
- `get_admin()`: Retrieve contract administrator
- `is_initialized()`: Check initialization status
- `require_initialized()`: Validate initialization
- `get_timestamp()`: Get validated timestamp
- `generate_transaction_id()`: Create unique transaction IDs
- `emit_error_event()`: Standardized error event emission
- `check_rate_limit()`: Rate limiting functionality

### ErrorHelpers
- `create_context()`: Create error context for logging
- `should_log()`: Determine if error should be logged
- `retry_strategy()`: Get suggested retry strategy

### EventEmit
- `operation_started()`: Emit operation start events
- `operation_completed()`: Emit operation completion events
- `operation_failed()`: Emit operation failure events

## Testing Framework

### Testing Utilities
- `setup_test_env()`: Standardized test environment setup
- `create_test_context()`: Create test error contexts
- `assert_error()`: Assert specific error occurrence
- `assert_success()`: Assert successful operation

### Test Coverage
- **Error Code Conversion**: All error codes properly convert
- **Category Classification**: Errors correctly categorized
- **Severity Assessment**: Proper severity assignment
- **Recoverability Logic**: Correct recoverability determination
- **Retry Strategy Mapping**: Appropriate retry suggestions
- **Macro Functionality**: All macros work correctly
- **Integration Scenarios**: End-to-end error handling
- **Edge Cases**: Boundary and unusual conditions

## Usage Examples

### Basic Error Handling
```rust
use crate::{std_error, StellarSpendError};

pub fn some_function(env: &Env, amount: i128) -> Result<(), StellarSpendError> {
    validate_amount!(env, amount);
    
    if amount > 1000 {
        std_error!(env, StellarSpendError::AmountExceedsLimit);
    }
    
    Ok(())
}
```

### Advanced Error Handling with Context
```rust
use crate::{std_error, StellarSpendError, ErrorHelpers};

pub fn advanced_function(env: &Env, user: &Address) -> Result<(), StellarSpendError> {
    let context = ErrorHelpers::create_context(
        env,
        1100,
        "MyContract",
        "advanced_function",
        Vec::new(env),
        Map::new(env),
    );
    
    if !is_authorized(user) {
        std_error!(env, StellarSpendError::Unauthorized, context);
    }
    
    Ok(())
}
```

### Error Documentation Lookup
```rust
use crate::ErrorDocumentation;

pub fn get_error_info(env: &Env, error_code: u32) {
    if let Some(doc) = ErrorDocumentation::get_documentation(env, error_code) {
        println!("Error: {}", doc.name);
        println!("Description: {}", doc.description);
        println!("Severity: {:?}", doc.severity);
        println!("Recoverable: {}", doc.recoverable);
        
        for cause in doc.causes.iter() {
            println!("Cause: {}", cause);
        }
        
        for solution in doc.solutions.iter() {
            println!("Solution: {}", solution);
        }
    }
}
```

### Rate Limiting
```rust
use crate::ContractUtils;

pub fn rate_limited_operation(env: &Env, user: &Address) -> Result<(), StellarSpendError> {
    ContractUtils::check_rate_limit(env, user, "my_operation", 5, 60)?;
    
    // Perform the operation
    perform_operation();
    
    Ok(())
}
```

## Integration Guidelines

### Contract Integration
1. **Import Standard Library**: `use crate::{std_error, StellarSpendError};`
2. **Replace Error Enums**: Use `StellarSpendError` instead of custom enums
3. **Use Standardized Macros**: Replace manual error checks with macros
4. **Implement Standard Trait**: Use `StandardContract` trait for consistency
5. **Emit Standard Events**: Use `EventEmit` for consistent events

### Error Handling Best Practices
1. **Validate Early**: Use validation macros at function start
2. **Provide Context**: Include relevant context information
3. **Use Safe Arithmetic**: Prevent overflow/underflow
4. **Log Appropriately**: Use logging decisions for important errors
5. **Document Clearly**: Use descriptive error messages

### Migration Strategy
1. **Phase 1**: Add error standardization library
2. **Phase 2**: Replace string-based errors gradually
3. **Phase 3**: Update all contracts to use standardized errors
4. **Phase 4**: Add comprehensive testing
5. **Phase 5**: Update documentation and examples

## Files Created/Modified

- **`contracts/errors.rs`** (600+ lines) - Complete error standardization system
- **`contracts/lib.rs`** (400+ lines) - Standardized library with macros and utilities
- **`tests/error_tests.rs`** (500+ lines) - Comprehensive error scenario tests

## Performance Considerations

### Memory Efficiency
- **Enum Optimization**: Error enums use minimal memory
- **String Interning**: Common strings shared
- **Lazy Documentation**: Documentation loaded on demand
- **Compact Storage**: Efficient error context storage

### Execution Efficiency
- **Macro Expansion**: Compile-time error validation
- **Inline Functions**: Common utilities inlined
- **Early Returns**: Fast failure paths
- **Minimal Allocations**: Efficient string handling

## Security Considerations

### Error Information Disclosure
- **Sanitized Messages**: No sensitive data in error messages
- **Consistent Responses**: Standardized error format
- **Rate Limiting**: Error enumeration protection
- **Audit Trail**: Complete error logging

### Attack Prevention
- **Input Validation**: Comprehensive parameter checking
- **Boundary Protection**: Overflow/underflow prevention
- **Authorization Checks**: Consistent permission validation
- **Resource Limits**: Protection against abuse

## Benefits Achieved

### 1. Consistency
- **Unified Error Codes**: Same error meanings across contracts
- **Standardized Patterns**: Consistent error handling approaches
- **Common Documentation**: Centralized error information
- **Uniform Testing**: Standardized test patterns

### 2. Maintainability
- **Centralized Management**: Single source of error definitions
- **Easy Updates**: Changes propagate to all contracts
- **Type Safety**: Compile-time error validation
- **Clear Semantics**: Well-defined error meanings

### 3. Developer Experience
- **Helpful Macros**: Simplified error handling
- **Rich Documentation**: Comprehensive error information
- **Testing Support**: Built-in testing utilities
- **IDE Integration**: Better autocomplete and documentation

### 4. User Experience
- **Clear Error Messages**: Understandable error descriptions
- **Actionable Solutions**: Specific fix recommendations
- **Consistent Format**: Predictable error responses
- **Recovery Guidance**: Clear retry instructions

The comprehensive error standardization system provides a robust, maintainable, and user-friendly approach to error handling across all StellarSpend contracts, replacing inconsistent string-based errors with a standardized, well-documented, and thoroughly tested system.
