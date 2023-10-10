fn main() -> io::Result<()> {
    let input_path = "input.csv";
    let output_path = "output.json";

    // Read CSV file
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);

    // Transform data and serialize to JSON
    let records: Vec<Record> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| serde_json::from_str(&line).ok())
        .collect();

    // Write JSON data to output file
    let output_file = File::create(output_path)?;
    serde_json::to_writer(output_file, &records)?;

    // Create an Airflow client
    let client = AirflowClient::new()?;

    // Create an Airflow task
    let task = Task::new("upload_data", None)?;

    // Upload data to Airflow
    upload_data_to_airflow(&client, &task, &records)?;

    Ok(())
}
