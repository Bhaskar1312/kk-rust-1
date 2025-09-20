use std::env 

fn main() -> Result<(), String> {
    let var_value = env::var("NUMBER").map_err(|_| "Env var not found".toString())?;
    let num: i32 = var_value.parse().map_err(|_| "Failed to parse as number".to_string())?;

     println!("Value: {}", num);
    Ok(())
}