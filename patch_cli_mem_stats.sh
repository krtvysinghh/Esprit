sed -i '' -e 's/fail("Failed to compress memory.");\
                }\
            }\
        }/fail("Failed to compress memory.");\
                }\
            }\
        }\
\
        Commands::MemoryStats => {\
            let count = esprit_memory::count().unwrap_or(0);\
            ok(\&format!("Memory currently holds {count} entries."));\
        }/g' apps/esprit-cli/src/main.rs
