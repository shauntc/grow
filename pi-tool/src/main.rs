use std::process;

use anyhow::Result;
use clap::Parser;

mod options {
    const TARGET: &str = "aarch64-unknown-linux-gnu";
    const PI_URL: &str = "pi-grow.local";
    const PI_USER: &str = "shaun";
    const BIN_NAME: &str = "pi";

    #[derive(Debug, clap::Subcommand)]
    #[command()]
    pub enum SubCommand {
        /// Build the 'pi' binary for the rapsberry pi
        Build,
        /// Upload the 'pi' binary to the raspberry pi
        Upload,
        /// Run the 'pi' binary on the raspberry pi
        Run,
        /// Kill the 'pi' binary on the raspberry pi
        Kill,
        /// Build, upload, enable execution, and run the 'pi' binary on the raspberry pi
        Dev,
        /// Upload and run the 'pi' binary on the raspberry pi
        Deploy,
    }

    #[derive(Debug, clap::Parser)]
    #[command()]
    pub struct Args {
        #[clap(subcommand)]
        pub command: SubCommand,
        #[clap(long, default_value = TARGET)]
        pub target: String,
        #[clap(long, default_value = PI_URL)]
        pub pi_url: String,
        #[clap(long, default_value = PI_USER)]
        pub pi_user: String,
        #[clap(short, long, default_value = BIN_NAME)]
        pub bin_name: String,
        #[clap(short, long, default_value = "false")]
        pub release: bool,
    }
}

enum Actions {
    Build,
    Upload,
    EnableExecution,
    Run,
    Kill,
}

fn main() -> Result<()> {
    let options::Args {
        command,
        target,
        pi_url,
        pi_user,
        bin_name,
        release,
    } = options::Args::parse();

    use options::SubCommand as C;
    let actions = match command {
        C::Build => vec![Actions::Build],
        C::Upload => vec![Actions::Upload],
        C::Run => vec![Actions::EnableExecution, Actions::Run],
        C::Kill => vec![Actions::Kill],
        C::Dev => vec![
            Actions::Build,
            Actions::Kill,
            Actions::Upload,
            Actions::EnableExecution,
            Actions::Run,
        ],
        C::Deploy => vec![Actions::Upload, Actions::EnableExecution, Actions::Run],
    };

    for action in actions {
        match action {
            Actions::Build => build(&target, &bin_name, release)?,
            Actions::Upload => upload(
                &target,
                ssh_address(&pi_url, &pi_user).as_str(),
                &bin_name,
                release,
            )?,
            Actions::EnableExecution => enable_execution(
                ssh_address(&pi_url, &pi_user).as_str(),
                &pi_bin_path(&bin_name),
            )?,
            Actions::Run => run(
                ssh_address(&pi_url, &pi_user).as_str(),
                &pi_bin_path(&bin_name),
            )?,
            Actions::Kill => kill(
                ssh_address(&pi_url, &pi_user).as_str(),
                &pi_bin_path(&bin_name),
            )?,
        }
    }

    Ok(())
}

fn build(target: &str, bin_name: &str, release: bool) -> Result<()> {
    println!(
        "Building for '{}' in {} using 'cross'",
        target,
        if release { "release" } else { "debug" }
    );

    let mut child_process = process::Command::new("cross")
        .args(["build", "--target", target, "-p", bin_name])
        .arg(if release { "--release" } else { "" })
        .stdout(process::Stdio::inherit())
        .spawn()?;

    child_process.wait()?;

    Ok(())
}

fn upload(target: &str, pi_address: &str, bin_name: &str, release: bool) -> Result<()> {
    println!(
        "Uploading '{}' to '{}' in {}",
        bin_name,
        pi_address,
        if release { "release" } else { "debug" }
    );
    let file_path = file_name(target, bin_name, release);
    let target_path = format!("{}:{}", pi_address, pi_bin_path(bin_name));

    let mut child_process = process::Command::new("scp")
        .args([file_path, target_path])
        .stdout(process::Stdio::inherit())
        .spawn()?;

    child_process.wait()?;

    Ok(())
}

fn enable_execution(pi_address: &str, pi_bin_path: &str) -> Result<()> {
    println!("Elevating '{}' on '{}'", pi_bin_path, pi_address);

    let mut child_process = process::Command::new("ssh")
        .args([pi_address, "chmod", "+x", pi_bin_path])
        .stdout(process::Stdio::inherit())
        .spawn()?;

    child_process.wait()?;

    Ok(())
}

fn run(pi_address: &str, pi_bin_path: &str) -> Result<()> {
    println!("Running '{}' on '{}'", pi_bin_path, pi_address);

    let mut child_process = process::Command::new("ssh")
        .args([pi_address, pi_bin_path])
        .stdout(process::Stdio::inherit())
        .spawn()?;

    child_process.wait()?;

    Ok(())
}

fn kill(pi_address: &str, pi_bin_path: &str) -> Result<()> {
    println!("Killing '{}' on '{}'", pi_bin_path, pi_address);

    let mut child_process = process::Command::new("ssh")
        .args([pi_address, "killall", pi_bin_path])
        .stdout(process::Stdio::inherit())
        .spawn()?;

    child_process.wait()?;

    Ok(())
}

fn ssh_address(pi_address: &str, pi_user: &str) -> String {
    format!("{}@{}", pi_user, pi_address)
}

fn file_name(target: &str, bin_name: &str, release: bool) -> String {
    format!(
        "target/{}/{}/{}",
        target,
        if release { "release" } else { "debug" },
        bin_name
    )
}

fn pi_bin_path(bin_name: &str) -> String {
    format!("/tmp/{}", bin_name)
}
