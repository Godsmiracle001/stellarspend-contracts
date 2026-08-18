# Gas Reference

Gas varies by Soroban protocol version, footprint, and argument size. Benchmark deployed builds with `soroban contract invoke` before setting production limits.

| Operation family | Relative cost |
|---|---|
| Read-only configuration | Low |
| Single balance or goal update | Low to medium |
| Policy validation with proof | Medium to high |
| Batch payment or rewards | High; scales with recipients |
| Audit and activity pagination | Medium; scales with records |
