pub fn help_msg() -> &'static str {
    r#"Usage: rusty-jvm [options] <mainclass> [args...]

Options:
    -D<name>=<value>  Set a system property
    -X<option>        JVM options
    -XX:<option>      Advanced JVM options
    --<option>        Java launcher options
    -<option>         Java standard options
    -h, --help        Show this help message

Installation options:
    --install         Download and install standard libraries
    --purge           Remove all versions of installed rusty-jvm standard libraries
    --yes             Automatically say "yes" to prompts
"#
}
