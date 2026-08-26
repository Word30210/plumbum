pub mod manifest;

pub fn display_err(result: anyhow::Result<()>, header: &str) {
    if let Err(err) = result {
        if header.is_empty() {
            eprintln!("{}\n", err);
        } else {
            eprintln!("{}: {}\n", header, err);
        }

        let cause = err.chain().skip(1).collect::<Vec<_>>();

        if !cause.is_empty() {
            eprintln!("caused by");
            for err in cause {
                eprintln!("\t- {}", err)
            }
        }

        let backtrace = err.backtrace();
        match backtrace.status() {
            std::backtrace::BacktraceStatus::Disabled => {
                eprintln!("\nhelp: set RUST_BACKTRACE=1 for backtrace")
            }
            std::backtrace::BacktraceStatus::Captured => eprintln!("\nbacktrace:\n{}", backtrace),
            _ => eprintln!("\nbacktrace: not captured"),
        }
    }
}
