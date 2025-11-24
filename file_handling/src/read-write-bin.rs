// Only required imports here
use std::fs::File;
use std::io::{ self, Read, Write};

fn main() -> io::Result<()> {  // Should return an IO Result type
    let mut file = File::create("binary_data.bin")?;  // Create the binary file
    file.write_all(&[1, 2, 3, 4, 5])?;  // Write data to file

    let mut file = File::open("binary_data.bin")?;  // Open the binary file
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;  // Read the file data to buffer using read to end function

    println!("Binary data: {:?}", buffer);  // Print the buffer
    // A final statement to make this program work
    Ok(())
}
