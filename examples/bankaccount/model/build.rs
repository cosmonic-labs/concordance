fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    // Generate markdown docs
    concordance_gen::generate_doc("../bankaccount-model.ttl".into(), "./docs".into())?;
  
    Ok(())
}