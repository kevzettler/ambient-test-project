((rustic-mode . ((eval . (setq-local lsp-rust-analyzer-cargo-target "wasm32-wasi"))
                 (eval . (setq-local lsp-rust-analyzer-cargo-watch-args ["--features" "client server"]))
                 (eval . (setq-local lsp-rust-features ["client" "server"])))))
