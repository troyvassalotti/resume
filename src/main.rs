use std::fs;
use std::path::PathBuf;
use std::result::Result;

use argh::FromArgs;
use minijinja::{syntax::SyntaxConfig, Environment};

use chrono::format::ParseError;
use chrono::{Datelike, NaiveDate};

/// A small application that renders a MiniJina template.
#[derive(FromArgs)]
struct Cli {
    /// the path to a JSON file with the context
    #[argh(option, short = 'c', long = "context")]
    context: PathBuf,

    /// the path to a template file that should be rendered
    #[argh(option, short = 't', long = "template")]
    template: PathBuf,
}

fn mmyyyy(value: &str) -> Result<String, ParseError> {
    let date = NaiveDate::parse_from_str(value, "%Y-%m-%d")?;
    let year = date.year();
    let month = date.month();

    Ok(year.to_string())
}

// todo: glob add_templates for anything in includes
fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = argh::from_env();

    let mut env = Environment::new();
    env.add_template("style.css", include_str!("_includes/style.css"))
        .unwrap();
    env.add_template("sprites.html", include_str!("_includes/sprites.html"))
        .unwrap();
    env.add_filter("mmyyyy", mmyyyy);
    env.set_syntax(
        SyntaxConfig::builder()
            .line_statement_prefix("#")
            .line_comment_prefix("##")
            .build()
            .unwrap(),
    );
    let source = fs::read_to_string(&cli.template)?;
    let name = cli.template.file_name().unwrap().to_str().unwrap();
    env.add_template(name, &source)?;

    let ctx: serde_json::Value = serde_json::from_slice(&fs::read(&cli.context)?)?;

    let tmpl = env.get_template(name).unwrap();
    println!("{}", tmpl.render(ctx)?);

    Ok(())
}

fn main() {
    execute().unwrap();
}
