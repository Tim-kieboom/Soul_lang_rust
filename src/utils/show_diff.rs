/// Asserts that two expressions are equal, printing a side-by-side diff if they are not.
///
/// The macro compares `$left` and `$right` using `==`.  
/// If they differ, the output from [`show_str_diff`] is displayed, formatting
/// the left and right values in a table and highlighting differences.
///
/// # Arguments
/// * `$left` - The left-hand expression to compare.
/// * `$right` - The right-hand expression to compare.
/// * `$msg` (optional) - A custom message to append to the diff output.
///
/// # Examples
/// ```
/// use soul_lang_rust::assert_eq_show_diff;
/// 
/// #[test]
/// fn same_values() {
///     let a = [1, 2, 3];
///     let b = [1, 2, 3];
///     assert_eq_show_diff(a, b)
/// }
/// 
/// #[test]
/// #[should_panic]
/// fn diffrent_values() {
///     let a = [1, 2, 3];
///     let b = [67, 68, 69];
///     assert_eq_show_diff(a, b)
/// }
/// ```
#[macro_export]
macro_rules! assert_eq_show_diff {
	($left:expr, $right:expr) => {
		assert!($left == $right, "{}", $crate::utils::show_diff::show_str_diff(format!("{:#?}", $left).as_str(), format!("{:#?}", $right).as_str()))
	};
    
    ($left:expr, $right:expr, $msg:expr) => {
		assert!($left == $right, "{}\n{}", $crate::utils::show_diff::show_str_diff(format!("{:#?}", $left).as_str(), format!("{:#?}", $right).as_str()), $msg)
	};
}

/// Produces a formatted side-by-side string comparison of two multiline strings.
///
/// This function splits `expected` and `got` into lines, prints them alongside each other,
/// and highlights character-level differences with a caret (`^`).
///
/// # Arguments
/// * `expected` - The expected string (left column).
/// * `got` - The actual string (right column).
///
/// # Returns
/// A string containing a table of compared lines, with differences marked beneath each differing line.
///
/// # Example
/// ```
/// use soul_lang_rust::utils::show_diff::show_str_diff;
/// println!("{}", show_str_diff("hello world", "helo word"));
/// ```
pub fn show_str_diff(expected: &str, got: &str) -> String {
    let exp_lines: Vec<&str> = expected.lines().collect();
    let got_lines: Vec<&str> = got.lines().collect();
    let max_lines = exp_lines.len().max(got_lines.len());

	let width_left = exp_lines.iter().map(|s| s.len()).max().unwrap_or(0);
	let width_right = got_lines.iter().map(|s| s.len()).max().unwrap_or(0);
	let column_width = width_left.max(width_right);

    let mut result = String::new();

    result.push_str(&format!("{:<width$} | {:<width$}\n",
        "Left:",
        "Right:",
        width = column_width)
    );
    result.push_str(&format!("{:-<width$}-+{:-<width$}\n",
        "",
        "",
        width = column_width)
    );

    for i in 0..max_lines {
        let exp = exp_lines.get(i).unwrap_or(&"");
        let got = got_lines.get(i).unwrap_or(&"");

        result.push_str(&format!("{:<width$} | {:<width$}\n",
            exp,
            got,
            width = column_width)
        );

        if exp != got {
            let exp_highlight = generate_diff_marker(exp, got);
            let got_highlight = generate_diff_marker(got, exp);

			let has_exp = exp_highlight.trim().len() > 0;
			let has_got = got_highlight.trim().len() > 0;

			if has_exp || has_got {
				result.push_str(&format!(
					"{:<width$} | {:<width$}\n",
					exp_highlight,
					got_highlight,
					width = column_width
				));
			}
        }
    }

    result
}

/// Produces a highlighted multiline string with line numbers and marked spans.
///
/// This is useful for pointing out specific ranges (spans) in source code or logs.
/// It prints the original lines along with underneath caret (`^`) markers to show
/// the highlighted ranges.
///
/// # Arguments
/// * `start_line` - The starting line number.
/// * `lines` - A slice of strings representing each line of text.
/// * `spans` - A list of tuples `(line_number, start, end)` specifying highlighted ranges.
///              - `line_number` is the absolute line number.
///              - `start` and `end` are character indices.
///
/// # Returns
/// A string with numbered lines and highlighted caret markers.
///
/// # Example
/// ```
/// use soul_lang_rust::utils::show_diff::generate_highlighted_string;
/// let lines = vec!["hello world".to_string(), "foo bar".to_string()];
/// let spans = vec![(1, 6, 11)]; // highlight "world"
/// println!("{}", generate_highlighted_string(1, &lines, &spans));
/// ```
pub fn generate_highlighted_string(
    start_line: usize,
    lines: &[String],
    spans: &[(usize, usize, usize)],
) -> String {
    let mut result = String::new();

    let max_line_number = lines.len();
    let number_width = (max_line_number + start_line).to_string().len();

    for (line_idx, line) in lines.iter().enumerate() {
        let line_number = start_line + line_idx;

        let mut line_spans = vec![];

        for &(span_line, start, end) in spans {
            if span_line == line_number {
                let local_start = start.min(line.len());
                let local_end = end.min(line.len());
                if local_start < local_end {
                    line_spans.push((local_start, local_end));
                }
            }
        }

        if line_spans.is_empty() {
            result.push_str(&format!("{:width$} | {}", line_number, line, width = number_width));
            result.push('\n');
            continue;
        }

        line_spans.sort();
        let mut merged: Vec<(usize, usize)> = vec![];

        for (start, end) in line_spans {
            if let Some(last) = merged.last_mut() {
                if start <= last.1 {
                    last.1 = last.1.max(end);
                    continue;
                }
            }
            merged.push((start, end));
        }

        let mut caret_line = vec![' '; line.len()];
        for (start, end) in merged {
            for i in start..end {
                if i < caret_line.len() {
                    caret_line[i] = '^';
                }
            }
        }

        let gutter = format!("{:width$} |", "", width = number_width);
        result.push_str(&gutter);
        result.push('\n');
        result.push_str(&format!("{:width$} | {}", line_number, line, width = number_width));
        result.push('\n');
        result.push_str(&format!(
            "{:width$} | {}",
            "",
            caret_line.iter().collect::<String>(),
            width = number_width
        ));
        result.push('\n');
    }

    result
}

fn generate_diff_marker(a: &str, b: &str) -> String {
    let max_len = a.chars().count().max(b.chars().count());
    let mut marker = String::new();

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    for i in 0..max_len {
        let a_ch = a_chars.get(i);
        let b_ch = b_chars.get(i);

        if a_ch != b_ch {
            marker.push('^');
        } else if a_ch.is_some() {
            marker.push(' ');
        } else {
            marker.push('^');
        }
    }

    marker
}










































