/// Returns a `String` containing the LaTeX code for the specified table
///
/// `output_vals` must have exactly `2^input_cols` rows, otherwise this will return an error
pub fn generate_table(
    input_cols: i32,
    output_vals: &mut Vec<Vec<i32>>,
    col_names: Vec<&str>,
) -> String {
    format!(
        "\\begin{{table*}}{{}}\n    \\begin{{tabular}}{{ {} }}\n        {} \\\\ \n        \\hline \n{}\n    \\end{{tabular}}\n\\end{{table*}}",
        (0..(input_cols + output_vals[0].len() as i32))
            .enumerate()
            .map(
                |(i, _)| if i == input_cols as usize { "| c" } else { " c" }
            ).collect::<Vec<&str>>()
            .join(" |")
            .as_str(),

        col_names.join(" & ").as_str(),

        (0..2_i32.pow(input_cols as u32))
            .map(
                |n| format!(
                    "{n:0amount$b}", amount = input_cols as usize
                ).trim()
                    .chars()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" & ")
                    + " & "
                    + output_vals.get(n as usize)
                        .unwrap()
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<String>>()
                        .join(" & ")
                        .as_str()
            )
            .map(|v| String::from("        ") + v.as_str())
            .collect::<Vec<String>>()
            .join(" \\\\ \n")
            .as_str()
    )
}
