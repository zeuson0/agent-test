fn main() -> shadow_rs::SdResult<()> {
    tonic_build::compile_protos("./protos/keyprovider.proto")?;
    tonic_build::compile_protos("./protos/getresource.proto")?;
    tonic_build::compile_protos("./protos/getevidence.proto")?;

    shadow_rs::new()
}