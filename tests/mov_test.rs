use std::{
    env::current_dir,
    fs::{self, File},
    io::Write,
    process::Command,
};

#[test]
fn disasm_listings() {
    fs::remove_dir_all("tests/output").ok();
    fs::create_dir("tests/output").unwrap();

    for dir_entry in fs::read_dir("tests/listings").unwrap().flatten() {
        let name = dir_entry.file_name().to_str().unwrap().to_owned();
        println!("\n\nTesting {}", name);

        let input_machine_code = fs::read(dir_entry.path()).unwrap();
        let asm = sim8086::disassemble(&input_machine_code);

        let path = current_dir().unwrap().join("tests/output").join(&name);
        let mut f = File::create(path.with_extension("asm")).unwrap();
        f.write_all(asm.as_bytes()).unwrap();

        let status = Command::new("nasm")
            .arg(path.with_extension("asm"))
            .spawn()
            .expect("Failed to start NASM")
            .wait()
            .unwrap();
        if status.code().unwrap() != 0 {
            panic!("NASM exit with an error");
        }

        let output_machine_code = fs::read(&path).unwrap();

        // for (i, (input, output)) in input_machine_code
        //     .iter()
        //     .zip(output_machine_code)
        //     .enumerate()
        // {
        //     assert_eq!(*input, output, "byte {i}");
        // }
        assert_eq!(input_machine_code, output_machine_code);
    }
}
