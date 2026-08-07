use esprit_bench::measure;

fn main() -> anyhow::Result<()> {
    for b in [
        measure("startup", || Ok(()))?,
        measure("search", || Ok(()))?,
        measure("semantic", || Ok(()))?,
    ] {
        println!("{:<12} {:>6} ms", b.name, b.elapsed_ms);
    }

    Ok(())
}
