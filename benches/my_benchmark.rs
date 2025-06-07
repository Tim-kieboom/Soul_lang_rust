use criterion::{criterion_group, criterion_main, Criterion};
use soul_lang_rust::{meta_data::meta_data::MetaData, run_compiler::run_compiler, run_options::{run_options::RunOptions, show_output::ShowOutputs, show_times::ShowTimes}, tokenizer::{file_line::FileLine, tokenizer::tokenize_file}};

const TEST_FILE: &str = r#"

sum(i32 one, i32 two) i32
{
	return one + two
// text etxt
}

// sdfrghsdf

main() 
{

/*
ertghserth
uytrduy
poujyitd
adsrfg
*/

	Print("hello world\n")
	stringArray := ["1", "2", "3", "4", "5", "6"]
	i32 result = sum(1, /*comment*/2)
	result += 1; result -= -1 
	result = \
		2

	Println(result)

	if true {
		return 0
	}
	else {
		return 1
	}
}
"#;

const TEST_HELLO_WORLD_FILE: &str = r#"
main() int {
	Println("hello world")
	Println("hello world")
	Println("hello world")
	Println("hello world")
	Println("hello world")
	Println("hello world")
	Println("hello world")
	Println("hello world")
	return 0
}
"#;

fn bench_tokenize_file(c: &mut Criterion) {
	
	let estimated_token_count = TEST_FILE.matches(" ").count() as u64;
	let source_file = TEST_FILE.split("\n")
		.enumerate()
		.map(|(i, line)| FileLine{text: line.to_string(), line_number: i as u64})
		.collect::<Vec<_>>();
	
	let mut meta_data = MetaData::new();

    c.bench_function(
		"tokenize file", 
		|b| b.iter(|| tokenize_file(source_file.clone(), estimated_token_count, &mut meta_data)),
	);

	let mut source_file = Vec::from([FileLine{text: "main() int {".to_string(), line_number: 0}]);
	let end = 1000;
	for i in 1..end {
		source_file.push(FileLine{text: "\tPrintln()".to_string(), line_number: i});
	}
	source_file.push(FileLine{text: "}".to_string(), line_number: end+1});
	let estimated_token_count = source_file.iter().map(|line| line.text.matches(" ").count() as u64).sum();

	c.bench_function(
		"stressTest tokenize file", 
		|b| b.iter(|| tokenize_file(source_file.clone(), estimated_token_count, &mut meta_data)),
	);
}

fn bench_hello_world_compiled(c: &mut Criterion) {
	
	c.bench_function(
		"compile hello_world.soul", 
		|b| b.iter(|| 
			run_compiler(RunOptions {
				file_path: TEST_HELLO_WORLD_FILE.to_string(),
				is_file_raw_str: true,
				is_compiled: true,
				show_outputs: ShowOutputs::SHOW_NONE,
				show_times: ShowTimes::SHOW_NONE,
				is_garbage_collected: false,
			})
			.expect("error in compiler while running bench tests")
		)
	);
}

criterion_group!(benches, bench_tokenize_file, bench_hello_world_compiled);
criterion_main!(benches);