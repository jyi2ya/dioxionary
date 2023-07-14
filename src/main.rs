use clap::CommandFactory;
use dioxionary::{
    cli::{Action, Cli, Parser},
    error::Result,
    history, list_dicts, query, repl,
};

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    if let Some(shell) = cli.completions {
        clap_complete::generate(shell, &mut Cli::command(), "dioxionary", &mut std::io::stdout());
        std::process::exit(0);
    }

    if let Some(action) = cli.action {
        match action {
            Action::Count => history::count_history(),
            Action::List(t) => history::list_history(t.type_, t.sort, t.table, t.column),
            Action::Lookup(w) => {
                if let Some(word) = w.word {
                    query(
                        w.online,
                        w.local_first,
                        w.exact_search,
                        word,
                        &w.local,
                        w.read_aloud,
                    )
                } else if !w.non_interactive {
                    repl(
                        w.online,
                        w.local_first,
                        w.exact_search,
                        &w.local,
                        w.read_aloud,
                    )
                } else {
                    Ok(())
                }
            }
            Action::Dicts => list_dicts(),
        }
    } else if let Some(word) = cli.word {
        query(
            cli.online,
            cli.local_first,
            cli.exact_search,
            word,
            &cli.local,
            cli.read_aloud,
        )
    } else if !cli.non_interactive {
        repl(
            cli.online,
            cli.local_first,
            cli.exact_search,
            &cli.local,
            cli.read_aloud,
        )
    } else {
        Ok(())
    }
}
