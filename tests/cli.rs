use assert_cmd::prelude::*; // komutlara methods ekleme
use predicates::prelude::*; // asserrions yazmak için kullanılır
use std::process::Command; // komutları çalıştırma

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Dosya okunamadı"));

    Ok(())
}

