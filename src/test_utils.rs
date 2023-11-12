use snarkvm::file::Manifest;
use snarkvm::prelude::*;

use snarkvm::synthesizer::Program;
use std::{fs, fs::File, io::Write, ops::Add, panic::catch_unwind, path::PathBuf, str::FromStr};

use crate::errors::{AvailError, AvailErrorType, AvailResult};

pub const WEAK_PASSWORD: &str = "password";
pub const STRONG_PASSWORD: &str = "ywZ9&377DQd5";
pub const TESTNET_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

pub const IMPORT_PROGRAM: &str = "
import credits.aleo;
program aleo_test.aleo;

function test:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
";

pub const FINALIZE_TEST_PROGRAM: &str = "program finalize_test.aleo;

mapping monotonic_counter:
    // Counter key
    key id as u32.public;
    // Counter value
    value counter as u32.public;

function increase_counter:
    // Counter index
    input r0 as u32.public;
    // Value to increment by
    input r1 as u32.public;
    finalize r0 r1;

finalize increase_counter:
    // Counter index
    input r0 as u32.public;
    // Value to increment by
    input r1 as u32.public;
    // Get or initialize counter key
    get.or_use monotonic_counter[r0] 0u32 into r2;
    // Add r1 to into the existing counter value
    add r1 r2 into r3;
    // Set r3 into account[r0];
    set r3 into monotonic_counter[r0];
";

pub const CREDITS_IMPORT_TEST_PROGRAM: &str = "import credits.aleo;
program credits_import_test.aleo;

function test:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
";

pub const HELLO_PROGRAM: &str = "program hello.aleo;

function hello:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
";

pub const HELLO_PROGRAM_2: &str = "program hello.aleo;

function hello:
    input r0 as u32.public;
    input r1 as u32.private;
    mul r0 r1 into r2;
    output r2 as u32.private;
";

pub const GENERIC_PROGRAM_BODY: &str = "

function fabulous:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
";

pub const MULTIPLY_PROGRAM: &str =
    "// The 'multiply_test.aleo' program which is imported by the 'double_test.aleo' program.
program multiply_test.aleo;

function multiply:
    input r0 as u32.public;
    input r1 as u32.private;
    mul r0 r1 into r2;
    output r2 as u32.private;
";

pub const MULTIPLY_IMPORT_PROGRAM: &str =
    "// The 'double_test.aleo' program that uses a single import from another program to perform doubling.
import multiply_test.aleo;

program double_test.aleo;

function double_it:
    input r0 as u32.private;
    call multiply_test.aleo/multiply 2u32 r0 into r1;
    output r1 as u32.private;
";

pub const RECORD_2000000001_MICROCREDITS: &str = r"{
  owner: aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3.private,
  microcredits: 2000000001u64.private,
  _nonce: 440655410641037118713377218645355605135385337348439127168929531052605977026group.public
}";

pub const RECORD_5_MICROCREDITS: &str = r"{
  owner: aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3.private,
  microcredits: 5u64.private,
  _nonce: 3700202890700295811197086261814785945731964545546334348117582517467189701159group.public
}";

/// Get a random program id
pub fn random_program_id(len: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();

    let program: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    program.add(".aleo")
}

/// Get a random program
pub fn random_program() -> Program<Testnet3> {
    let random_program = String::from("program ")
        .add(&random_program_id(15))
        .add(";")
        .add(GENERIC_PROGRAM_BODY);
    Program::<Testnet3>::from_str(&random_program).unwrap()
}

/// Create temp directory with test data
pub fn setup_directory(
    directory_name: &str,
    main_program: &str,
    imports: Vec<(&str, &str)>,
) -> AvailResult<PathBuf> {
    // Crate a temporary directory for the test.
    let directory = std::env::temp_dir().join(directory_name);

    catch_unwind(|| {
        let _ = &directory
            .exists()
            .then(|| fs::remove_dir_all(&directory).unwrap());
        fs::create_dir(&directory).unwrap();

        let imports_directory = directory.join("imports");
        fs::create_dir(directory.join("imports")).unwrap();
        let program = Program::<Testnet3>::from_str(main_program).unwrap();
        let program_id = program.id();

        // Create the manifest file.
        Manifest::create(&directory, program_id).unwrap();

        let mut main = File::create(directory.join("main.aleo")).unwrap();
        main.write_all(main_program.as_bytes()).unwrap();

        imports.into_iter().for_each(|(name, program)| {
            let mut file = File::create(imports_directory.join(name)).unwrap();
            file.write_all(program.as_bytes()).unwrap();
        });
    })
    .map_err(|_| {
        AvailError::new(
            AvailErrorType::Internal,
            "Failed to create test directory".to_string(),
            "".to_string(),
        )
    })?;
    Ok(directory)
}

/// Teardown temp directory
pub fn teardown_directory(directory: &PathBuf) {
    if directory.exists() {
        fs::remove_dir_all(directory).unwrap();
    }
}
