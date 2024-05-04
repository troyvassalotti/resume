use argh::FromArgs;
use chrono::{Datelike, NaiveDate};
use fs_extra::{copy_items, dir};
use minify_html::{minify, Cfg};
use minijinja::{path_loader, syntax::SyntaxConfig, AutoEscape, Environment};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;

/// A small application that renders a MiniJina template.
#[derive(FromArgs)]
struct Cli {
    /// the path to a JSON file with the context
    #[argh(option, short = 'c', long = "context")]
    context: PathBuf,

    /// the path to a template file that pshould be rendered
    #[argh(option, short = 't', long = "template")]
    template: PathBuf,
}

fn pretty_date(date: &str) -> String {
    let val = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let year = val.year();
    let month = val.month();
    format!("{month:02}/{year}")
}

fn create_env() -> Environment<'static> {
    let mut env = Environment::new();

    env.set_auto_escape_callback(|_name| AutoEscape::None);
    env.set_loader(path_loader("src/_includes"));
    env.add_filter("pretty_date", pretty_date);
    env.set_syntax(
        SyntaxConfig::builder()
            .line_statement_prefix("#")
            .line_comment_prefix("##")
            .build()
            .unwrap(),
    );

    env
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = argh::from_env();
    let source = fs::read_to_string(&cli.template)?;
    let name = cli.template.file_name().unwrap().to_str().unwrap();

    let mut env = create_env();
    env.add_template(name, &source)?;

    let ctx: serde_json::Value = serde_json::from_slice(&fs::read(&cli.context)?)?;
    let tmpl = env.get_template(name).unwrap();
    let contents = tmpl.render(ctx)?;

    let mut cfg = Cfg::new();
    cfg.keep_comments = true;
    cfg.keep_closing_tags = true;
    cfg.do_not_minify_doctype = true;
    cfg.minify_css = true;
    cfg.keep_spaces_between_attributes = true;
    cfg.keep_html_and_head_opening_tags = true;
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;
    let minified = minify(contents.as_bytes(), &cfg);

    fs::create_dir_all("dist")?;
    let mut file = File::create("dist/index.html")?;
    file.write_all(&minified)?;

    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    let from_paths = vec!["public/static", "public/favicon.ico"];
    copy_items(&from_paths, "dist", &options)?;

    Ok(())
}

fn main() {
    build().unwrap();
}
