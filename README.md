# Wirefilter

This is an execution engine for Wireshark-style filters.

It contains public APIs for parsing filter syntax, compiling them into
an executable IR and, finally, executing filters against provided values.

# Example

```rust
use wirefilter::{Scheme, ExecutionContext, Type};

// Create a map of possible filter fields
let scheme: Scheme = (&[
    ("http.method", Type::Bytes),
    ("http.ua", Type::Bytes),
    ("port", Type::Int),
]).into();

// Create a filter
let ast = scheme.parse(
    r#"http.method != "POST" && not http.ua matches "(googlebot|facebook)" && port in {80 443}"#
)?;

println!("Parsed filter representation: {:?}", ast);

let filter = ast.compile();

// Set runtime field values to test the filter against
let mut ctx = ExecutionContext::new(&scheme);

ctx.set_field_value("http.method", "GET")?;

ctx.set_field_value(
    "http.ua",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/44.0.2403.157 Safari/537.36"
)?;

ctx.set_field_value("port", 443)?;

// Execute the filter with given runtime values
println!("Filter matches: {:?}", filter.execute(&ctx)?); // true

// Amend one of the runtime values and execute the filter again
ctx.set_field_value("port", 8080)?;

println!("Filter matches: {:?}", filter.execute(&ctx)?); // false
```