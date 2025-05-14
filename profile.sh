RISC0_PPROF_OUT=./profile.pb RUST_LOG=info RISC0_DEV_MODE=1 RISC0_INFO=1 cargo run -- --mode Prove --output-file "./spres.json" --path "./minimal" --query-file "./queries/query.rq"
