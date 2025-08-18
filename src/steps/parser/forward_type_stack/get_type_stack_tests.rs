use std::collections::{HashMap};

use crate::{assert_eq_show_diff, steps::{parser::forward_type_stack::get_type_stack::get_scope_from_type_stack, step_interfaces::i_parser::{abstract_syntax_tree::{expression::Ident, soul_type::type_kind::{TypeKind, TypeSize}}, scope::{ExternalPages, InnerScope, ScopeVisibility}}, tokenizer::tokenizer_test}, utils::serde_multi_ref::MultiRefPool};

const TEST_FILE: &str = r#"
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

#[test]
fn get_type_stack_should_work() {
    let mut stream = tokenizer_test::get_test_tokenizer(TEST_FILE)
        .inspect_err(|err| panic!("{:?}", err))
        .unwrap().stream;

    let scopes = get_scope_from_type_stack(&mut stream, MultiRefPool::new(), ExternalPages::new(), "test".into())
        .inspect_err(|err| panic!("{:?}", err))
        .unwrap();

    let should_be = vec![
        InnerScope {
            parent_index: None,
            children: vec![1, 2, 3, 4],
            self_index: 0,
            symbols: HashMap::from([
                ("str".into(), TypeKind::Str),
                ("i8".into(), TypeKind::Int(TypeSize::Bit8)),
                ("none".into(), TypeKind::None),
                ("bool".into(), TypeKind::Bool),
                ("int".into(), TypeKind::SystemInt),
                ("uint".into(), TypeKind::SystemUint),
                ("i16".into(), TypeKind::Int(TypeSize::Bit16)),
                ("f16".into(), TypeKind::Float(TypeSize::Bit16)),
                ("untypedUint".into(), TypeKind::UntypedUint),
                ("f8".into(), TypeKind::Float(TypeSize::Bit8)),
                ("i64".into(), TypeKind::Int(TypeSize::Bit64)),
                ("untypedInt".into(), TypeKind::UntypedInt),
                ("u8".into(), TypeKind::Uint(TypeSize::Bit8)),
                ("f64".into(), TypeKind::Float(TypeSize::Bit64)),
                ("char".into(), TypeKind::Char(TypeSize::Bit8)),
                ("u16".into(), TypeKind::Uint(TypeSize::Bit16)),
                ("i32".into(), TypeKind::Int(TypeSize::Bit32)),
                ("untypedFloat".into(), TypeKind::UntypedFloat),
                ("u64".into(), TypeKind::Uint(TypeSize::Bit64)),
                ("u32".into(), TypeKind::Uint(TypeSize::Bit32)),
                ("f32".into(), TypeKind::Float(TypeSize::Bit32)),
                ("Strable".into(), TypeKind::Trait(Ident("Strable".into()))),
                ("StringData".into(), TypeKind::Struct(Ident("StringData".into()))),
                ("String".into(), TypeKind::Class(Ident("String".into()))),
                ("stringGlobal".into(), TypeKind::TypeDefed(Ident("stringGlobal".into()))),
            ]),
            visibility_mode: ScopeVisibility::GlobalOnly,
        },
        InnerScope {
            parent_index: Some(0),
            children: vec![],
            self_index: 1,
            symbols: HashMap::from([]),
            visibility_mode: ScopeVisibility::All,
        },
        InnerScope {
            parent_index: Some(
                0,
            ),
            children: vec![],
            self_index: 2,
            symbols: HashMap::from([]),
            visibility_mode: ScopeVisibility::All,
        },
        InnerScope {
            parent_index: Some(
                0,
            ),
            children: vec![],
            self_index: 3,
            symbols: HashMap::from([]),
            visibility_mode: ScopeVisibility::All,
        },
        InnerScope {
            parent_index: Some(
                0,
            ),
            children: vec![5,7],
            self_index: 4,
            symbols: HashMap::from([("string1".into(), TypeKind::TypeDefed(Ident("string1".into())))]),
            visibility_mode: ScopeVisibility::All,
        },
        InnerScope {
            parent_index: Some(
                4,
            ),
            children: vec![6],
            self_index: 5,
            symbols: HashMap::from([("Foo".into(), TypeKind::Struct(Ident("Foo".into())))]),
            visibility_mode: ScopeVisibility::All,
        },
        InnerScope {
            parent_index: Some(
                5,
            ),
            children: vec![],
            self_index: 6,
            symbols: HashMap::from([("string3".into(), TypeKind::TypeDefed(Ident("string3".into())))]),
            visibility_mode: ScopeVisibility::All,
        },
        InnerScope {
            parent_index: Some(
                4,
            ),
            children: vec![8,9],
            self_index: 7,
            symbols: HashMap::from([("string4".into(), TypeKind::TypeDefed(Ident("string4".into())))]),
            visibility_mode: ScopeVisibility::All,
        },
        InnerScope {
            parent_index: Some(
                7,
            ),
            children: vec![],
            self_index: 8,
            symbols: HashMap::from([("string5".into(), TypeKind::TypeDefed(Ident("string5".into())))]),
            visibility_mode: ScopeVisibility::All,
        },
        InnerScope {
            parent_index: Some(
                7,
            ),
            children: vec![],
            self_index: 9,
            symbols: HashMap::from([("string6".into(), TypeKind::TypeDefed(Ident("string6".into())))]),
            visibility_mode: ScopeVisibility::All,
        },
    ];
    assert_eq_show_diff!(scopes.get_types(), &should_be);
}








