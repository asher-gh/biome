use similar::{utils::diff_lines, Algorithm, ChangeTag};
use std::sync::Mutex;
use std::{env, fmt::Write, fs::write, os::raw::c_int, str::FromStr, sync::Once};

use serde::Serialize;

#[derive(Debug, PartialEq, Eq)]
enum ReportType {
    Json,
    Markdown,
}

#[derive(Debug, Clone, Default, Serialize)]
struct SingleFileMetricData {
    filename: String,
    single_file_compatibility: f64,
    #[serde(skip)]
    diff: Option<String>,
}

impl SingleFileMetricData {
    fn is_compatible(&self) -> bool {
        (self.single_file_compatibility * 100_f64) >= 100.00
    }
}

#[derive(Clone, Debug, Default, Serialize)]
struct PrettierCompatibilityMetricData {
    file_based_average_prettier_similarity: f64,
    line_based_average_prettier_similarity: f64,
    files: Vec<SingleFileMetricData>,
}

impl FromStr for ReportType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "markdown" => Ok(Self::Markdown),
            _ => Err("Only `json` and `markdown` are supported".to_string()),
        }
    }
}

struct DiffReportItem {
    file_name: &'static str,
    biome_formatted_result: String,
    prettier_formatted_result: String,
}
pub struct DiffReport {
    state: Mutex<Vec<DiffReportItem>>,
}

impl DiffReport {
    pub fn get() -> &'static Self {
        static REPORTER: DiffReport = DiffReport {
            state: Mutex::new(Vec::new()),
        };

        // Use an atomic Once to register an exit callback the first time any
        // testing thread requests an instance of the Reporter
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            // Import the atexit function from libc
            extern "C" {
                fn atexit(f: extern "C" fn()) -> c_int;
            }

            // Trampoline function into the reporter printing logic with the
            // correct extern C ABI
            extern "C" fn print_report() {
                REPORTER.print();
            }

            // Register the print_report function to be called when the process exits
            unsafe {
                atexit(print_report);
            }
        });

        &REPORTER
    }

    pub fn report(
        &self,
        file_name: &'static str,
        biome_formatted_result: &str,
        prettier_formatted_result: &str,
    ) {
        match env::var("REPORT_PRETTIER") {
            Ok(value) if value == "1" => {
                if !Self::is_ignored(file_name) {
                    self.state.lock().unwrap().push(DiffReportItem {
                        file_name,
                        biome_formatted_result: biome_formatted_result.to_owned(),
                        prettier_formatted_result: prettier_formatted_result.to_owned(),
                    });
                }
            }
            _ => {}
        }
    }

    fn is_ignored(file_name: &str) -> bool {
        // ignore unstable syntaxes and embedded languages in template literals
        let patterns = [
            // Experimental syntaxes
            "arrows-bind",
            "async-do-expressions",
            "async-do-expressions",
            "babylon-extensions",
            "decimal",
            "do-expressions",
            "export-default-from",
            "function-bind",
            "module-block",
            "partial-application",
            "pipeline",
            "record",
            "throw-expression",
            "throw_expression",
            "v8intrinsic",
            "v8_intrinsic",
            "bind-expressions",
            "js/objects/expression.js",
            "destructuring-private-fields",
            "/do/",
            "export-extension",
            "js/export-default/escaped/default-escaped.js", // re-export of default export is unstable
            "js/tuple",
            "deferred-import-evaluation", // `import defer` syntax
            "source-phase-imports",       // `import source` syntax
            "import-reflection",          // `import module` syntax
            "explicit-resource-management",
            // Embedded languages in template literals
            "html-like",
            "js/multiparser",
            "js/range/issue-7082.js",
            "js/template-literals/css-prop.js",
            "js/template-literals/styled",
            "js/last-argument-expansion/embed.js",
            "typescript/as/as-const-embedded.ts",
            "js/last-argument-expansion/embed.js",
            "typescript/as/as-const-embedded.ts",
            "styled-components",
        ];

        patterns.iter().any(|pattern| file_name.contains(pattern))
    }

    fn print(&self) {
        if let Some(report) = biome_rowan::check_live() {
            panic!("\n{report}")
        }
        // Only create the report file if the REPORT_PRETTIER
        // environment variable is set to 1
        match env::var("REPORT_PRETTIER") {
            Ok(value) if value == "1" => {
                let report_type = match env::var("REPORT_TYPE") {
                    Ok(value) => ReportType::from_str(&value).unwrap(),
                    _ => ReportType::Markdown,
                };
                let incompatible_only =
                    matches!(env::var("INCOMPATIBLE_ONLY"), Ok(value) if value == "1");

                let report_filename = match env::var("REPORT_FILENAME") {
                    Ok(value) => value,
                    _ => match report_type {
                        ReportType::Json => {
                            if incompatible_only {
                                "report_incompatible.json".to_string()
                            } else {
                                "report.json".to_string()
                            }
                        }
                        ReportType::Markdown => {
                            if incompatible_only {
                                "report_incompatible.md".to_string()
                            } else {
                                "report.md".to_string()
                            }
                        }
                    },
                };
                self.report_prettier(report_type, report_filename, incompatible_only);
            }
            _ => {}
        }
    }

    fn report_prettier(
        &self,
        report_type: ReportType,
        report_filename: String,
        incompatible_only: bool,
    ) {
        let mut state = self.state.lock().unwrap();
        state.sort_by_key(|DiffReportItem { file_name, .. }| *file_name);

        let mut report_metric_data = PrettierCompatibilityMetricData::default();
        let mut file_ratio_sum = 0_f64;
        let mut total_lines = 0;
        let mut total_matched_lines = 0;
        let mut file_count = 0;

        for DiffReportItem {
            file_name,
            biome_formatted_result,
            prettier_formatted_result,
        } in state.iter()
        {
            file_count += 1;

            let biome_lines = biome_formatted_result.lines().count();
            let prettier_lines = prettier_formatted_result.lines().count();

            let (matched_lines, ratio, diff) =
                if biome_formatted_result == prettier_formatted_result {
                    (biome_lines, 1f64, None)
                } else {
                    let mut matched_lines = 0;
                    let mut diff = String::new();

                    for (tag, line) in diff_lines(
                        Algorithm::default(),
                        prettier_formatted_result,
                        biome_formatted_result,
                    ) {
                        if matches!(tag, ChangeTag::Equal) {
                            matched_lines += 1;
                        }

                        let line = line.strip_suffix('\n').unwrap_or(line);
                        writeln!(diff, "{}{}", tag, line).unwrap();
                    }

                    let ratio = matched_lines as f64 / biome_lines.max(prettier_lines) as f64;

                    (matched_lines, ratio, Some(diff))
                };

            total_lines += biome_lines.max(prettier_lines);
            total_matched_lines += matched_lines;
            file_ratio_sum += ratio;

            let single_file_metric_data = SingleFileMetricData {
                diff,
                filename: (*file_name).to_string(),
                single_file_compatibility: ratio,
            };

            // We'll skip compatible tests and only track incompatible ones
            if incompatible_only && single_file_metric_data.is_compatible() {
                continue;
            }

            report_metric_data.files.push(single_file_metric_data);
        }

        report_metric_data.file_based_average_prettier_similarity =
            file_ratio_sum / f64::from(file_count);
        report_metric_data.line_based_average_prettier_similarity =
            total_matched_lines as f64 / total_lines as f64;

        match report_type {
            ReportType::Json => self.report_json(report_filename, report_metric_data),
            ReportType::Markdown => self.report_markdown(report_filename, report_metric_data),
        }
    }

    fn report_markdown(
        &self,
        report_filename: String,
        report_metric_data: PrettierCompatibilityMetricData,
    ) {
        let mut report = String::new();

        for SingleFileMetricData {
            filename,
            single_file_compatibility,
            diff,
        } in report_metric_data.files.iter()
        {
            writeln!(report, "# {}", filename).unwrap();

            if let Some(diff) = diff {
                writeln!(report, "```diff").unwrap();
                writeln!(report, "{diff}").unwrap();
                writeln!(report, "```").unwrap()
            }
            writeln!(report).unwrap();
            writeln!(
                report,
                "**Prettier Similarity**: {:.2}%",
                single_file_compatibility * 100_f64
            )
            .unwrap();
            writeln!(report).unwrap();
            writeln!(report).unwrap();
        }

        let mut header = String::from("# Overall Metrics\n\n");

        writeln!(
            header,
            "**Average compatibility**: {:.2}",
            report_metric_data.file_based_average_prettier_similarity * 100_f64,
        )
        .unwrap();

        header.push_str(
            r"
<details>
    <summary>Definition</summary>

    $$average = \frac\{\sum_{file}^\{files}compatibility_\{file}}\{files}$$
</details>

",
        );

        write!(
            header,
            "**Compatible lines**: {:.2}",
            report_metric_data.line_based_average_prettier_similarity * 100_f64
        )
        .unwrap();

        header.push_str(
                r"

<details>
    <summary>Definition</summary>

    $$average = \frac{\sum_{file}^{files}matching\_lines_{file}}{max(lines_{rome}, lines_{prettier})}$$
</details>

[Metric definition discussion](https://github.com/rome/tools/issues/2555#issuecomment-1124787893)
",
            );

        let report = format!("{header}\n\n{report}");
        write(report_filename, report).unwrap();
    }

    fn report_json(
        &self,
        report_filename: String,
        report_metric_data: PrettierCompatibilityMetricData,
    ) {
        let json_content = serde_json::to_string(&report_metric_data).unwrap();
        write(report_filename, json_content).unwrap();
    }
}
