use timelord::{
    core::{clear, log_error, Command},
    record::record,
    report::report_full,
};

fn main() {
    let command = extract(std::env::args().collect());
    match command {
        Some(command) => handle(command),
        None => show_help("No command provided."),
    }
}

fn handle(command: Command) {
    match command.identifier.as_str() {
        "help" => handle_help(),
        "clear" => handle_clear(),
        "record" => handle_record(command.arguments),
        "report" => handle_report(),
        other => show_help(format!("Unknown command: {}", other).as_str()),
    }
}

fn show_help(cause: &str) {
    println!("{}", cause);
    println!("Available commands: ");
    println!("timelord clear - erases all recorded data");
    println!("timelord record <hours> <task> <budget> - records hours on task to budget");
    println!("timelord report - shows recorded hours and budgets report");
    println!("timelord help - shows this text");
}

fn extract(args: Vec<String>) -> Option<Command> {
    if args.len() < 2 {
        return None;
    }
    return Some(Command {
        identifier: args.get(1).unwrap().to_string(),
        arguments: args[2..args.len()].to_vec(),
    });
}

fn handle_help() {
    show_help("Requested help");
}

fn handle_report() {
    report_full();
}

fn handle_record(arguments: Vec<String>) {
    if arguments.len() < 3 {
        log_error("At least three arguments expected. Use 'timelord help' to see command usage.");
    }
    record(
        arguments
            .get(0)
            .unwrap()
            .parse::<u8>()
            .expect("hours argument can not be parsed to a number"),
        arguments.get(1).unwrap(),
        arguments.get(2).unwrap(),
    );
}

fn handle_clear() {
    clear();
}
