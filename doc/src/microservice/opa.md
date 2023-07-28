# Working with Open Policy Agent
The microservice can utilize the function:
```rust,noplayground
pub async fn opa_eval<I>(input: Serialize) -> Result<serde_json::Value>
```
from `src/policy/policy.rs`
which sends the `input` to an opa_server or uses it as input to evaluate a to `.wasm` compiled `.rego` file dependant on the set [enviornment variables](./environment.md).

