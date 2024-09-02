use plotly::{Bar, Plot};
const EXCLUDED_LANG: [&str; 11] = [
    "HTML",
    "TOML",
    "YAML",
    "C Header",
    "C++ Header",
    "CMake",
    "JSON",
    "Makefile",
    "Jupyter Notebooks",
    "Plain Text",
    "Markdown",
];
fn main() -> anyhow::Result<()> {
    use tokei::{Config, Languages};

    let paths = &["."];
    let excluded = &["target"];
    let config = Config::default();
    let mut languages = Languages::new();

    languages.get_statistics(paths, excluded, &config);

    let mut data = Vec::new();
    for (lang, lines) in languages {
        let name = lang.name();
        if EXCLUDED_LANG.contains(&name) {
            continue;
        }
        data.push((lang.name(), lines.code));
    }
    data.sort_by(|a, b| b.1.partial_cmp(&a.1).expect("error"));

    let names: Vec<&str> = data.iter().map(|(name, _)| *name).collect();
    let values: Vec<usize> = data.iter().map(|(_, value)| *value).collect();

    let total: usize = values.iter().sum();
    let percentages: Vec<String> = values
        .iter()
        .map(|&value| format!("{:.1}%", (value as f64 / total as f64) * 100.0))
        .collect();
    let trace = Bar::new(names, values).text_array(percentages);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();

    Ok(())
}
