use crate::meta_data::{borrow_checker::borrow_checker::{BorrowCheckedTrait, BorrowChecker, BorrowId}, scope_and_var::scope::ScopeId};

fn add_scope(checker: &mut BorrowChecker, id: u64) -> ScopeId {
    let scope = ScopeId(id);
    checker.open_scope(&scope)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();

    scope
}

#[test]
fn test_declare_owner() {
    let mut checker = BorrowChecker::new();
    let scope = add_scope(&mut checker, 0);

    let var = BorrowId("x", &scope);
    checker.declare_owner(&var)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();

    let duplicate = checker.declare_owner(&var);
    assert!(duplicate.is_err());

    /*
        x := 1
    */
}

#[test]
fn test_const_borrow() {
    let mut checker = BorrowChecker::new();
    let scope = add_scope(&mut checker, 0);


    let x = BorrowId("x", &scope);
    let x_ref = BorrowId("x_ref", &scope);

    checker.declare_owner(&x).unwrap();
    checker.borrow_const(&x_ref, &x)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();

    /*
        x := 1
        x_ref := @x 
    */
}

#[test]
fn test_mut_borrow() {
    let mut checker = BorrowChecker::new();
    let scope = add_scope(&mut checker, 0);


    let x = BorrowId("x", &scope);
    let x_mut_ref = BorrowId("x_mut_ref", &scope);

    checker.declare_owner(&x).unwrap();
    checker.borrow_mut(&x_mut_ref, &x)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();

    let another_mut_ref = BorrowId("x_mut_ref2", &scope);
    let result = checker.borrow_mut(&another_mut_ref, &x);
    assert!(result.is_err());

    /*
        x := 1
        x_mut_ref := &x 
    */
}

#[test]
fn test_move_owner() {
    let mut checker = BorrowChecker::new();
    let scope = add_scope(&mut checker, 0);


    let a = BorrowId("a", &scope);
    let b = BorrowId("b", &scope);

    checker.declare_owner(&a).unwrap();
    checker.declare_owner(&b).unwrap();

    checker.move_owner(&a, &b)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();

    let res = checker.borrow_const(&BorrowId("a_ref", &scope), &a);
    assert!(res.is_err(), "moved-from owner should be invalid");

    /*
        a := 1
        b := 1
        b = a
        a_ref := @a <-- should fail
    */
}

#[test]
fn test_drop_owner() {
    let mut checker = BorrowChecker::new();
    let scope = add_scope(&mut checker, 0);


    let a = BorrowId("a", &scope);
    checker.declare_owner(&a).unwrap();

    checker.drop_owner(&a)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();

    let res = checker.borrow_const(&BorrowId("a_ref", &scope), &a);
    assert!(res.is_err());

    /*
        a := 1
        drop(a)
        a_ref := a <-- should fail
    */
}

#[test]
fn test_close_scope_and_cleanup() {
    let mut checker = BorrowChecker::new();
    let scope = add_scope(&mut checker, 0);


    let a = BorrowId("a", &scope);
    let b = BorrowId("b", &scope);

    checker.declare_owner(&a).unwrap();
    checker.declare_owner(&b).unwrap();

    let deleted = checker.close_scope(&scope)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();

    assert!(deleted.contains(&"a".to_string()));
    assert!(deleted.contains(&"b".to_string()));

    /*
    {
        a := 1
        b := 1
    } <-- drop a, b (DeleteList: [a, b])
    */
}

#[test]
fn test_nested_scopes_and_borrowing() {
    let mut checker = BorrowChecker::new();
    let inner = add_scope(&mut checker, 0);
    let outer = add_scope(&mut checker, 1);

    let x = BorrowId("x", &outer);
    checker.declare_owner(&x).unwrap();
    
    let x_ref = BorrowId("x_ref", &inner);
    checker.borrow_const(&x_ref, &x)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();

    let deleted = checker.close_scope(&inner)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();
    assert!(deleted.is_empty(), "x_ref has a parent and should not be cleaned up");

    let deleted_outer = checker.close_scope(&outer)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();
    assert!(deleted_outer.contains(&"x".to_string()));

    /*
    {
        x := 1

        {
            x_ref := @a
        } <-- drop x_ref (DeleteList: [])

    } <-- drop x (DeleteList: [x])
    */
}

#[test]
fn test_redeclare_after_drop() {
    let mut checker = BorrowChecker::new();
    let scope = ScopeId(0);
    checker.open_scope(&scope).unwrap();

    let a = BorrowId("a", &scope);
    checker.declare_owner(&a).unwrap();
    checker.drop_owner(&a).unwrap();

    let redeclare = checker.declare_owner(&a);
    assert!(redeclare.is_ok(), "Should allow re-declaring dropped variable in same scope, error: {:#?}", redeclare.unwrap_err());

    /*
        a := 1
        drop(1)
        a := 1
    */
}

#[test]
fn test_borrow_after_drop_fails() {
    let mut checker = BorrowChecker::new();
    let scope = ScopeId(0);
    checker.open_scope(&scope).unwrap();

    let a = BorrowId("a", &scope);
    let b = BorrowId("b", &scope);

    checker.declare_owner(&a).unwrap();
    checker.drop_owner(&a).unwrap();

    let result = checker.borrow_const(&b, &a);
    assert!(result.is_err(), "Borrowing a dropped owner should fail");

    /*
        a := 1
        drop(a)
        b := @a <-- should fail
    */
}

#[test]
fn test_borrow_after_move_fails() {
    let mut checker = BorrowChecker::new();
    let scope = ScopeId(0);
    checker.open_scope(&scope).unwrap();

    let a = BorrowId("a", &scope);
    let b = BorrowId("b", &scope);
    let a_ref = BorrowId("a_ref", &scope);

    checker.declare_owner(&a).unwrap();
    checker.declare_owner(&b).unwrap();

    checker.move_owner(&a, &b).unwrap();

    let res = checker.borrow_const(&a_ref, &a);
    assert!(res.is_err(), "Cannot borrow from moved variable");

    /*
        a := 1
        b := 1
        b = a
        a_ref := @a <-- should fail
    */
}

#[test]
fn test_drop_invalidates_const_refs() {
    let mut checker = BorrowChecker::new();
    let scope = ScopeId(0);
    checker.open_scope(&scope).unwrap();

    let a = BorrowId("a", &scope);
    let a_ref = BorrowId("a_ref", &scope);

    checker.declare_owner(&a).unwrap();
    checker.borrow_const(&a_ref, &a).unwrap();
    checker.drop_owner(&a).unwrap();

    let result = checker.borrow_const(&BorrowId("dummy", &scope), &a_ref);
    assert!(result.is_err(), "Reference from dropped owner should be invalid");

    /*
        a := 1
        a_ref := @a
        drop(a)
        dummy := @a_ref <-- should fail 
    */
}

#[test]
fn test_mut_borrow_after_const() {
    let mut checker = BorrowChecker::new();
    let scope = ScopeId(0);
    checker.open_scope(&scope).unwrap();

    let x = BorrowId("x", &scope);
    let x_ref = BorrowId("x_ref", &scope);
    let x_mut_ref = BorrowId("x_mut_ref", &scope);

    checker.declare_owner(&x).unwrap();
    checker.borrow_const(&x_ref, &x).unwrap();

    let _ = checker.borrow_mut(&x_mut_ref, &x)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap();

    /*
        x := 1
        x_ref := @x
        x_mut_ref := &x
    */
}





