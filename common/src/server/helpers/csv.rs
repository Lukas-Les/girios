use std::error::Error;
use std::fs::File;
use csv::Writer;


pub fn dump_as_csv(result: Vec<(String, &String)>, out_file_name: &str) -> Result<(), Box<dyn Error>>{

    let mut wtr = Writer::from_writer(File::create(out_file_name)?);

    // Write the header (if needed)
    wtr.write_record(&["path", "value"])?;

    // Write each entry in the vector to the CSV file
    for (key, value) in result {
        wtr.write_record(&[key, value.to_string()])?;
    }

    // Ensure the writer writes all data to the file
    wtr.flush()?;

    println!("Data has been written to {}", out_file_name);

    Ok(())
}
