pub fn show_str_diff(expected: &str, got: &str) -> String {
    
	fn char_to_byte_idx(s: &str, char_idx: usize) -> usize {
    	s.char_indices().nth(char_idx).map(|(i, _)| i).unwrap_or(s.len())
	}
	
	fn longest_line_char_count(s: &str) -> usize {
    	s.lines()
        	.map(|line| line.chars().count())
        	.max()
    	    .unwrap_or(0)
	}

	let exp_shift_size = longest_line_char_count(expected) + 6;
	let got_shift_size = longest_line_char_count(got) + 6;

	let expected_lines: Vec<_> = expected.lines().collect();
    let got_lines: Vec<_> = got.lines().collect();
    let max_lines = expected_lines.len().max(got_lines.len());

    println!("Line  | {:<exp_shift_size$} | {:<got_shift_size$}", "Expected", "Got");
    let exp_amount_minus = "-".repeat(exp_shift_size+2);
    let got_amount_minus = "-".repeat(got_shift_size+2);
	println!("------+{}+{}", exp_amount_minus, got_amount_minus);

	let mut lines = Vec::new();

    for i in 0..max_lines {
        let exp = expected_lines.get(i).unwrap_or(&"");
        let got = got_lines.get(i).unwrap_or(&"");
        let marker = if exp == got { " " } else { "!" };

        let diff_index = exp.chars()
            .zip(got.chars())
            .position(|(a, b)| a != b)
            .unwrap_or_else(|| exp.len().min(got.len()));

		let exp_diff_byte = char_to_byte_idx(exp, diff_index);
		let exp_next_byte = char_to_byte_idx(exp, diff_index + 1);

		let got_diff_byte = char_to_byte_idx(got, diff_index);
		let got_next_byte = char_to_byte_idx(got, diff_index + 1);

		let (exp_highlight, got_highlight) = if exp != got {
			(
				format!(
					"{}>>>{}<<<{}",
					&exp[..exp_diff_byte],
					&exp[exp_diff_byte..exp_next_byte],
					&exp[exp_next_byte..]
				),
				format!(
					"{}>>>{}<<<{}",
					&got[..got_diff_byte],
					&got[got_diff_byte..got_next_byte],
					&got[got_next_byte..]
				),
			)
		} else {
			(exp.to_string(), got.to_string())
		};

        lines.push(format!(
            "{:>4}{} | {:<exp_shift_size$} | {:<got_shift_size$}",
            i + 1,
            marker,
            exp_highlight,
            got_highlight
        ));
    }

	lines.join("\n")
}




















