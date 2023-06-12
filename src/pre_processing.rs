use crate::configuration::{Point, PointVector};
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Pre-processing step, receives database file name and returns its data normalized
pub fn pre_process_database(database_file_name: &str, collumns: &Vec<usize>) -> PointVector {

    println!("\nStarting pre process to {}", database_file_name);

    // converts database to str
    let database_csv_str: String = database_to_str(database_file_name);

    // converts str csv to point vector
    let mut point_vector: PointVector = csv_str_to_point_vector(database_csv_str, collumns);

    // normalizes the point vector
    normalize_point_vector(&mut point_vector);
    
    println!("Pre process Done!");
    return point_vector;
}

// reads a database file and converts it to string using Path that works for windows and linux environments
fn database_to_str(database_file_name: &str) -> String {
    let database_path = Path::new(".").join("datasets").join(database_file_name);
    let mut file_str = String::new();

    match database_path.to_str() {
        None => panic!("Error: Database_path is not a valid UTF-8 sequence"),
        Some(s) => println!("Path of file is {}", s),
    };

    match File::open(database_path) {
        Ok(mut file) => file.read_to_string(&mut file_str).unwrap(),
        Err(error) => panic!(
            "Error: A problem ocurred while opening the database {}: {:?}",
            database_file_name, error
        ),
    };

    return file_str;
}

// converts a string in csv format to PointVector
fn csv_str_to_point_vector(csv_str: String, collumns: &Vec<usize>) -> PointVector {
    let mut reader = csv::Reader::from_reader(csv_str.as_bytes());
    let mut point_vector: PointVector = Vec::new();
    let mut point_vector_pos: usize = 0;

    println!("Converting csv to point vector...");

    for raw_record in reader.records() {
        let record = match raw_record {
            Ok(raw_record) => raw_record,
            Err(error) => panic!(
                "Error: A problem ocurred while acessing csv records: {:?}",
                error
            ),
        };
        // parse given collumns, they must be floats
        point_vector.push(Vec::new());
        for col_pos in 0..collumns.len() {
            point_vector[point_vector_pos].push(record[collumns[col_pos]].parse::<f32>().unwrap());
        }
        point_vector_pos += 1;
    }

    return point_vector;
}

// normalize Points from a PointVector: O(3n) considering entry points not point vector
fn normalize_point_vector(point_vector: &mut PointVector) -> () {
    if point_vector.len() == 0 {
        panic!("Error: PointVector must have items");
    }

    println!("Normalizing data...");

    // initialize temporary points to place min and max values of each attribute
    let mut min_point_values: Point = vec![f32::MAX; point_vector[0].len() as usize];
    let mut max_point_values: Point = vec![f32::MIN; point_vector[0].len() as usize];
    let mut max_min_difference = vec![0.0; point_vector[0].len() as usize];

    // Iterates in points to get each attributes max and min: O(2n) considering entry points
    for point in point_vector.iter() {
        for point_pos in 0..point.len() {
            if point[point_pos] < min_point_values[point_pos] {
                min_point_values[point_pos] = point[point_pos];
            } else if point[point_pos] > max_point_values[point_pos] {
                max_point_values[point_pos] = point[point_pos];
            }
        }
    }

    // calculate the difference a single time for efficiency
    for point_pos in 0..max_min_difference.len() {
        max_min_difference[point_pos] = max_point_values[point_pos] - min_point_values[point_pos];
    }

    // Iterates in points to normalize: O(1n) considering entry points
    for vector_pos in 0..point_vector.len() {
        for point_pos in 0..point_vector[vector_pos].len() {
            point_vector[vector_pos][point_pos] = (point_vector[vector_pos][point_pos]
                - min_point_values[point_pos])
                / max_min_difference[point_pos];
        }
    }
}
