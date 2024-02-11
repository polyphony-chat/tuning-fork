#[cfg(not(test))]
use crate::CLI_ARGS;
use colored::{ColoredString, Colorize};

#[cfg(test)]
static CLI_ARGS: crate::Cli = crate::Cli {
    mode: crate::Mode::Server,
    port: 8192,
    verbose: false,
    waves: false,
};

pub struct Tests {
    tests: Vec<Test>,
}

impl Tests {
    pub fn add(
        &mut self,
        name: &str,
        description: &str,
        method: Box<dyn Fn(u16) -> Result<(), String>>,
    ) {
        self.tests.push(Test::new(name, description, method))
    }

    pub fn execute(&mut self) {
        for test in self.tests.iter_mut() {
            test.execute()
        }
    }

    pub fn execute_display(&mut self) {
        self.execute();
        for test in self.tests.iter() {
            println!("{}", test)
        }
    }
}

/// Represents a `stimmgabel` test acceptance case.
pub struct Test {
    pub name: String,
    pub description: String,
    pub method: Box<dyn Fn(u16) -> Result<(), String>>,
    result: Option<Result<(), String>>,
}

impl Test {
    pub fn new(
        name: &str,
        description: &str,
        method: Box<dyn Fn(u16) -> Result<(), String>>,
    ) -> Self {
        Test {
            name: name.to_string(),
            description: description.to_string(),
            method,
            result: None,
        }
    }

    /// Executes the method supplied in `self` and saves the [`Result`] inside of `self`.
    ///
    /// Consider using the [`Tests`] struct to execute and print a lot of tests at once.
    pub fn execute(&mut self) {
        match (self.method)(CLI_ARGS.port) {
            Ok(_) => self.result = Some(Ok(())),
            Err(e) => self.result = Some(Err(e)),
        }
    }
}

impl std::fmt::Display for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let component =
            |x: ColoredString| format!("{}{}{}", "[".bold().yellow(), x, "]".bold().yellow());
        if let Some(result) = &self.result {
            match result {
                Ok(_) => write!(
                    f,
                    "{} {}: Passed",
                    component("✓".bold().green()),
                    self.name.to_uppercase()
                ),
                Err(e) => write!(
                    f,
                    "{} {}: Failure: {}",
                    component("-".cyan().bold()),
                    self.name.to_uppercase(),
                    e
                ),
            }
        } else {
            write!(
                f,
                "{} {}: Skipped",
                component("-".red().bold()),
                self.name.to_uppercase()
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn port_is_power_of_two(port: u16) -> Result<(), String> {
        if port.is_power_of_two() {
            Ok(())
        } else {
            Err("The supplied port is not a power of two.".to_string())
        }
    }

    #[test]
    fn test_test_struct() {
        let mut test = Test::new(
            "My synchronous Test",
            "a synchronous test",
            Box::new(port_is_power_of_two),
        );

        test.execute();
        assert!(test.result.is_some());
        println!("{}", test);
    }
}
