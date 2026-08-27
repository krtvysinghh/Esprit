sed -i '' -e 's|MemoryStats,|MemoryStats,\
    /// Compress long-term chat memory to free up context\
    MemoryCompress,|g' apps/esprit-cli/src/main.rs

sed -i '' -e 's|Commands::MemoryStats => {|Commands::MemoryCompress => {\
            let sp = spinner("Compressing memory...");\
            let ai = esprit_ai::Ai::default_model().unwrap_or_else(|_| {\
                fail("AI model not loaded.");\
                std::process::exit(1);\
            });\
            let rows = esprit_memory::fetch_last(10).unwrap_or_default();\
            if rows.is_empty() {\
                sp.finish_and_clear();\
                ok("Memory is already empty.");\
            } else {\
                let mut ctx = String::new();\
                for (id, txt) in rows {\
                    ctx.push_str(\&format!("{id}: {txt}\\n"));\
                }\
                let prompt = format!("Summarize this chat history into a dense knowledge summary:\\n\\n{ctx}");\
                if let Ok(summary) = ai.ask(\&prompt) {\
                    let _ = esprit_memory::clear();\
                    let _ = esprit_memory::append("System", \&format!("Compressed Memory: {summary}"));\
                    sp.finish_and_clear();\
                    ok("Memory compressed successfully.");\
                } else {\
                    sp.finish_and_clear();\
                    fail("Failed to compress memory.");\
                }\
            }\
        }\
\
        Commands::MemoryStats => {|' apps/esprit-cli/src/main.rs
