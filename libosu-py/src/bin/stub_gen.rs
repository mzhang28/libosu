fn main() -> pyo3_stub_gen::Result<()> {
    let stub = libosu::stub_info()?;
    stub.generate()?;
    Ok(())
}
