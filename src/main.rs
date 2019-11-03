use std::{
    env,
    path::Path,
    io::{stdin,stdout, Write},
    process::{Command, Stdio, Child},
};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split("|").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let program = parts.next().unwrap();
            let args = parts;
            match program {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                },
                "exit" => return,
                program => {
                    let current_stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                    let current_stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(program)
                        .args(args)
                        .stdin(current_stdin)
                        .stdout(current_stdout)
                        .spawn();
                    
                    match output {
                        Ok(output) => {previous_command = Some(output)},
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    }
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}
