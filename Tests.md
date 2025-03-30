<!-- @format -->

# Integration tests

cargo test --test components

## Notes

Test Types Cheatsheet

Component File Tests

Fast (<1ms each)
Test internal logic
Prop validation
Simple rendering
Integration Tests

Medium speed
Verify DOM structure
Prop combinations
Accessibility checks
Runtime Tests

Slower (browser required)
User interactions
State changes
Complex workflows
🔄 Example Test Flow

Unit Test verifies class generation logic
Integration Test checks rendered HTML contains classes
Runtime Test confirms classes update during interaction
