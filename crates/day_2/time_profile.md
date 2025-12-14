# Performance Profile - Day 2

## Answers
- **Part 1**: 37314786486
- **Part 2**: 47477053982

## Timing Results

| Commit/Version | Part 1 Time | Part 2 Time | Notes |
|----------------|-------------|-------------|-------|
| b40c207 (Debug) | 141.651334ms | 618.100625ms | Debug build |
| b40c207 (Release) | 62.190709ms | 163.181416ms | Release build (optimized) |
| 264dd1f (Debug) | 82.18825ms | 147.82875ms | Debug build - Candidate generation approach |
| 264dd1f (Release) | 6.242208ms | 15.858208ms | Release build - Candidate generation approach |

## Notes

- Debug timings measured with `cargo run --bin day_2`
- Release timings measured with `cargo run --release --bin day_2`
- Input file: `crates/day_2/input.txt`
- System: macOS (darwin 23.4.0)
