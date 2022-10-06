pub(crate) fn escape(input: &str) -> String {
    String::from(input)
        .replace('\\', r#"\\"#)
        .replace('"', r#"\""#)
        .replace(';', r#"\;"#)
        .replace(',', r#"\,"#)
        .replace(':', r#"\:"#)
}
