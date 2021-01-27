use spirv_builder::SpirvBuilder;
use std::error::Error;

fn build_shader(path_to_create: &str) -> Result<(), Box<dyn Error>> {
    SpirvBuilder::new(path_to_create)
        .spirv_version(1, 0)
        .print_metadata(true)
        .memory_model(spirv_builder::MemoryModel::Vulkan)
        .build()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    build_shader("../shaders/d2-shader")?;
    Ok(())
}
