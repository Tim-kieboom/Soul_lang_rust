use std::collections::BTreeMap;

use crate::{steps::{parser::forward_type_stack::get_type_stack::{forward_declarde_type_stack}, step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{TypeKind}, tokenizer::tokenizer_test}, utils::show_diff::show_str_diff};

const TEST_FILE: &str = r#"
interface Istr {}
trait Strable {}
struct StringData {}
class String {}
type stringGlobal typeof str

main(str[]@ args) int {
	type string1 typeof str
	// innerSum(int a, int b) int {
	// 	return a + b
	// }

	if true {
		struct Foo {
			type string3 typeof str
		}
	}
	else {
		type string4 typeof str
		if true {
			type string5 typeof str
		}
		else {
			type string6 typeof str
		}
	}

	add := 1 
	Println("hello //world")
	Println(f"hello world {0}")
	Println(f"hello world {#0}")
}
"#;

const SHOULD_BE: &str = r#"[
    {
        "Istr": Interface(
            Ident(
                "Istr",
            ),
        ),
        "bool": Bool,
        "char": Char(
            Bit8,
        ),
        "f16": Float(
            Bit16,
        ),
        "f32": Float(
            Bit32,
        ),
        "f64": Float(
            Bit64,
        ),
        "f8": Float(
            Bit8,
        ),
        "i16": Int(
            Bit16,
        ),
        "i32": Int(
            Bit32,
        ),
        "i64": Int(
            Bit64,
        ),
        "i8": Int(
            Bit8,
        ),
        "int": SystemInt,
        "none": None,
        "str": Str,
        "u16": Uint(
            Bit16,
        ),
        "u32": Uint(
            Bit32,
        ),
        "u64": Uint(
            Bit64,
        ),
        "u8": Uint(
            Bit8,
        ),
        "uint": SystemUint,
        "untypedFloat": UntypedFloat,
        "untypedInt": UntypedInt,
        "untypedUint": UntypedUint,
    },
    {
        "Strable": Trait(
            Ident(
                "Strable",
            ),
        ),
    },
    {
        "StringData": Struct(
            Ident(
                "StringData",
            ),
        ),
    },
    {
        "String": Class(
            Ident(
                "String",
            ),
        ),
    },
    {
        "stringGlobal": Custom(
            Ident(
                "stringGlobal",
            ),
        ),
    },
    {
        "string1": Custom(
            Ident(
                "string1",
            ),
        ),
    },
    {
        "Foo": Struct(
            Ident(
                "Foo",
            ),
        ),
    },
    {
        "string3": Custom(
            Ident(
                "string3",
            ),
        ),
    },
    {
        "string4": Custom(
            Ident(
                "string4",
            ),
        ),
    },
    {
        "string5": Custom(
            Ident(
                "string5",
            ),
        ),
    },
    {
        "string6": Custom(
            Ident(
                "string6",
            ),
        ),
    },
]"#;

#[test]
fn get_type_stack_should_work() {
    let mut stream = tokenizer_test::get_test_tokenizer(TEST_FILE)
        .inspect_err(|err| panic!("{:?}", err))
        .unwrap().stream;

    let type_stack = forward_declarde_type_stack(&mut stream)
        .inspect_err(|err| panic!("{:?}", err))
        .unwrap();

    let type_stack_string = format!(
        "{:#?}", 
        type_stack.scopes
            .into_iter()
            .map(|scope| scope.symbols.into_iter().collect::<BTreeMap<String, TypeKind>>())
            .collect::<Vec<_>>()
    );
    
    assert!(
        type_stack_string == SHOULD_BE,
        "{}", show_str_diff(SHOULD_BE, type_stack_string.as_str())
    );
}








