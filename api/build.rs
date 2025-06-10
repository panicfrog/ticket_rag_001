use vergen::{Emitter, RustcBuilder, BuildBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = BuildBuilder::all_build()?;
    let rustc = RustcBuilder::all_rustc()?;
    
    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&rustc)?
        .emit()?;
        
    Ok(())
} 