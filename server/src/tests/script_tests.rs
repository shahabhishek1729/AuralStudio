use crate::file_utils;
use crate::transpiler::decompiler::Decompiler;

#[test]
fn test_script_basic() {
    let file_path = "src/tests/scripts/basic.rattle";
    let mut f = file_utils::File::new(file_path);
    f.parse();

    let rtl_source = f.read();

    let gt_path = "src/tests/scripts/basic.py";
    let mut f2 = file_utils::File::new(gt_path);
    // Stores tone contents of the file inside the `contents` attribute
    let _ = f2.read();

    let mut decompiler = Decompiler::new(&rtl_source).unwrap();
    decompiler.decompile().unwrap();

    let gen_path = "src/tests/scripts/basic_gen.py";
    let mut f3 = file_utils::File::new(gen_path);
    f3.write(&decompiler.py);

    // If something goes wrong, print out what transpiled program
    assert_eq!(f3, f2)
}

#[test]
fn test_script_advanced() {
    let file_path = "src/tests/scripts/advanced.rattle";
    let mut f = file_utils::File::new(file_path);
    f.parse();

    let rtl_source = f.read();

    let gt_path = "src/tests/scripts/advanced.py";
    let mut f2 = file_utils::File::new(gt_path);
    // Stores tone contents of the file inside the `contents` attribute
    let _ = f2.read();

    let mut decompiler = Decompiler::new(&rtl_source).unwrap();
    decompiler.decompile().unwrap();

    let gen_path = "src/tests/scripts/advanced_gen.py";
    let mut f3 = file_utils::File::new(gen_path);
    f3.write(&decompiler.py);

    // If something goes wrong, print out what transpiled program
    assert_eq!(f3, f2)
}

#[test]
fn test_script_password() {
    let file_path = "src/tests/scripts/password.rattle";
    let mut f = file_utils::File::new(file_path);
    f.parse();

    let rtl_source = f.read();

    let gt_path = "src/tests/scripts/password.py";
    let mut f2 = file_utils::File::new(gt_path);
    // Stores tone contents of the file inside the `contents` attribute
    let _ = f2.read();

    let mut decompiler = Decompiler::new(&rtl_source).unwrap();
    decompiler.decompile().unwrap();

    let gen_path = "src/tests/scripts/password_gen.py";
    let mut f3 = file_utils::File::new(gen_path);
    f3.write(&decompiler.py);

    // If something goes wrong, print out what transpiled program
    assert_eq!(f3, f2)
}

#[test]
fn test_script_parity() {
    let file_path = "src/tests/scripts/parity.rattle";
    let mut f = file_utils::File::new(file_path);
    f.parse();

    let rtl_source = f.read();

    let gt_path = "src/tests/scripts/parity.py";
    let mut f2 = file_utils::File::new(gt_path);
    // Stores tone contents of the file inside the `contents` attribute
    let _ = f2.read();

    let mut decompiler = Decompiler::new(&rtl_source).unwrap();
    decompiler.decompile().unwrap();

    let gen_path = "src/tests/scripts/parity_gen.py";
    let mut f3 = file_utils::File::new(gen_path);
    f3.write(&decompiler.py);

    // If something goes wrong, print out what transpiled program
    assert_eq!(f3, f2)
}

#[test]
fn test_script_linalg() {
    let file_path = "src/tests/scripts/linalg.rattle";
    let mut f = file_utils::File::new(file_path);
    f.parse();

    let rtl_source = f.read();

    let gt_path = "src/tests/scripts/linalg.py";
    let mut f2 = file_utils::File::new(gt_path);
    // Stores tone contents of the file inside the `contents` attribute
    let _ = f2.read();

    let mut decompiler = Decompiler::new(&rtl_source).unwrap();
    decompiler.decompile().unwrap();

    let gen_path = "src/tests/scripts/linalg_gen.py";
    let mut f3 = file_utils::File::new(gen_path);
    f3.write(&decompiler.py);

    // If something goes wrong, print out what transpiled program
    assert_eq!(f3, f2)
}
