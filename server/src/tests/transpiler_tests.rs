use crate::transpiler::decompiler::Decompiler;

#[test]
fn test_expr() {
    let expr = "x plus y greater than 5 plus 10 plus string how are you done";

    let mut decompiler = Decompiler::new(expr).unwrap();
    let s = decompiler.decompile_expr(false).unwrap();
    // dbg!(&decompiler.py);
    assert_eq!(s, "x + y > 5 + 10 + \"how are you\"");
}

#[test]
fn factorial_test() {
    let expr =
        "define factorial of x\nlet base be 1\nif x equals base\nreturn 1\ndone if\nreturn x times factorial of x minus 1 done";

    let mut decompiler = Decompiler::new(expr).unwrap();
    decompiler.decompile().unwrap();
    // dbg!(&decompiler.py);
    println!("{}", decompiler.py);
    assert_eq!(
        &decompiler.py,
        "def factorial(x):\n    base = 1\n    if x == base:\n        return 1\n        \n    return x * factorial(x - 1)"
    );
}

#[test]
fn quick_test() {
    dbg!("ANYTHING?");
    let expr = "from_mp3 of string x done done";

    let mut decompiler = Decompiler::new(expr).unwrap();
    let s = decompiler.decompile_calls_(false).unwrap();
    dbg!(&decompiler.py);
    assert_eq!(s, "from_mp3(\"x\")");
}

#[test]
fn test_expr2() {
    let expr = "main of x and y done plus 3";

    let mut decompiler = Decompiler::new(expr).unwrap();
    let s = decompiler.decompile_expr(false);
    dbg!(&s);
    dbg!(&decompiler.py);
    assert_eq!(s.unwrap(), "main(x, y) + 3");
}

#[test]
fn test_expr3() {
    // TODO: Figure out why and before false doesn't work
    let expr = "let x be both of inverse of main of how_are_you and y done done and false done";

    let mut decompiler = Decompiler::new(expr).unwrap();
    decompiler.decompile().unwrap();
    dbg!(&decompiler.py);
    assert_eq!(decompiler.py, "x = not main(how_are_you, y) and False");
}

#[test]
fn test_calls_base() {
    let expr = "main of x done";

    let mut decompiler = Decompiler::new(expr).unwrap();
    let s = decompiler.decompile_calls_(false).unwrap();
    dbg!(&decompiler.py);
    assert_eq!(s, "main(x)");
}

#[test]
fn test_calls_compound() {
    let expr = "main of how_are_you and x done";

    let mut decompiler = Decompiler::new(expr).unwrap();
    let s = decompiler.decompile_calls_(false).unwrap();
    dbg!(&decompiler.py);
    assert_eq!(s, "main(how_are_you, x)");
}

#[test]
fn test_lists() {
    let source = "list how are you doing today done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "[how, are, you, doing, today]");
}

#[test]
fn test_tuples() {
    let source = "tuple of 4 and tuple of 3 and string how are you done done done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "(4, (3, \"how are you\"))");
}

#[test]
fn test_dicts() {
    let source =
        "dictionary of 4 and tuple of 3 done and string how are you done and string i'm good done done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "{4: (3), \"how are you\": \"i'm good\"}");
}

#[test]
fn test_program() {
    let source = "define sum_list of lst\nlet l be len of lst done\nif l equals 0\nstring your list was empty done\ndone if\ndone define";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(
        decompiler.py,
        "def sum_list(lst):\n    l = len(lst)\n    if l == 0:\n        \"your list was empty\"\n        \n    "
    );
}

#[test]
fn test_imports() {
    let source = "grab pandas";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "import pandas");
}

#[test]
fn test_imports_alias() {
    let source = "grab pandas alias pd";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "import pandas as pd");
}

#[test]
fn another_test() {
    let source = "let x be 5\nif x modulo 2 equals 0 \noutput string even done\ndone if\notherwise\noutput string odd done\ndone otherwise ";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(
        decompiler.py,
        "x = 5\nif x % 2 == 0:\n    print(\"even\")\n    \nelse:\n    print(\"odd\")\n    "
    );
}

#[test]
fn test_prints() {
    let source = "output string hello world done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "print(\"hello world\")")
}

#[test]
fn test_calls_with_dots() {
    let source = "grab string\nlet x be list of 1 and 2 and escape string dot ascii_letters done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(
        decompiler.py,
        "import string\nx = [1, 2, string.ascii_letters]"
    )
}

#[test]
fn test_package_call() {
    let source =
        "louder_audio_file dot export of string example_louder.mp3 done and string mp3 done done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(
        decompiler.py,
        "louder_audio_file.export(\"example_louder.mp3\", \"mp3\")"
    );
}

#[test]
fn test_dots() {
    let source = "call a dot b dot c dot d done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "a.b.c.d()");
}

#[test]
fn test_named_args() {
    let source = "a dot b dot c dot d of josh be true done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "a.b.c.d(josh = True)");
}

#[test]
fn test_indexing1() {
    let source = "l at 3";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "l[3]");
}

#[test]
fn test_indexing2() {
    let source = "l at 3 plus 4";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "l[3] + 4");
}

#[test]
fn test_indexing3() {
    let source = "l at result of 3 plus 4 done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "l[3 + 4]");
}

#[test]
fn test_indexing4() {
    let source = "let l at result of 3 plus 4 done be 4";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "l[3 + 4] = 4");
}

#[test]
fn test_indexing5() {
    let source = "output l at 0";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "print(l[0])");
}

#[test]
fn test_classes() {
    let source =
        "type Dog\ndefine __init__ of self\ndone define\ndone type\n\nlet dog be Dog of done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(
        decompiler.py,
        "class Dog:\n    def __init__(self):\n        \n    \n\ndog = Dog()"
    );
}

#[test]
fn test_fancy_strings() {
    let source = "output string hello world space sign done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "print(\"hello world \")");
}

#[test]
fn test_quantity() {
    let source = "output 1 over quantity of 1 plus 3 done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "print(1 / (1 + 3))");
}

#[test]
fn test_quantity2() {
    let source = "output 1 over quantity of 1 plus 3 done done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "print(1 / (1 + 3))");
}

#[test]
fn test_star() {
    let source = "output star list of 1 and 2 and 3 done done";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "print(*[1, 2, 3])");
}

#[test]
fn test_asserts() {
    let source = "ensure 3 equals 3";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, "assert 3 == 3");
}

#[test]
fn test_linalg() {
    let source = "define inverse of m
define determinant of m
let x be m at 0 times m at 3
let y be m at 1 times m at 2
return x minus y
done define
define adjoint of m 
let result be list m at 3 0 minus m at 1 0 minus m at 2 m at 0 done 
return result
done define
done define";

    let py_source = "def inverse(m):
    def determinant(m):
        x = m[0] * m[3]
        y = m[1] * m[2]
        return x - y
        
    def adjoint(m):
        result = [m[3], 0 - m[1], 0 - m[2], m[0]]
        return result
        
    ";
    let mut decompiler = Decompiler::new(source).unwrap();
    decompiler.decompile().unwrap();
    println!("{}", decompiler.py);
    assert_eq!(decompiler.py, py_source);
}
