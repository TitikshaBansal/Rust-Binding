use cpdb_rs::frontend::Frontend;
use cpdb_rs::job::PrintJob;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    /// List available printers
    List,
    
    /// Submit a print job
    Print {
        printer: String,
        file: String,
        #[structopt(short, long)]
        copies: Option<u32>,
    },
}

fn main() -> anyhow::Result<()> {
    let cmd = Command::from_args();
    
    match cmd {
        Command::List => {
            let frontend = Frontend::global()?.lock()?;
            for printer in frontend.get_printers()? {
                println!("- {}", printer.name()?);
            }
        }
        Command::Print { printer, file, copies } => {
            let mut options = vec![];
            if let Some(c) = copies {
                options.push(("copies", c.to_string()));
            }
            
            let job = PrintJob::new(&printer, &options, &file)?;
            job.submit()?;
            println!("Submitted print job to {}", printer);
        }
    }
    
    Ok(())
}