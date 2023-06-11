use std::fs::File;
use std::io::Read;
use std::path::Path;

type Point = Vec<f32>; // Each Point is an item of the dataset
type PointVector = Vec<Point>; // PointVector is a collection of dataset items

fn main() -> () {
    let database_file_name: &str = "Iris.csv";
    pre_process_database(database_file_name);
}

// Pre-processing step, receives database file name and returns its data normalized
fn pre_process_database(database_file_name: &str) -> () {
    // converts database to str
    let database_csv_str: String = database_to_str(database_file_name);

    //println!("{}", database_csv_str);

    // converts str csv to point vector
    let mut point_vector: PointVector = csv_str_to_point_vector(database_csv_str);

    /*
    println!("Parsed csv to PointVector: ");
    for point in point_vector.clone() {
        println!("{:?}", point);
    }
    */

    // normalizes the point vector
    normalize_point_vector(&mut point_vector);

    println!("Normalize PointVector: ");
    for point in point_vector.clone() {
        print!("[");
        for point_pos in 0..point.len() {
            print!("{: ^6.2}", point[point_pos]);
        }
        println!("]");
    }

    // do you dbscan function bellow, this functions need to have its own modules for organization
    println!(
        "euclidian_distance({:?}, {:?}) = {}",
        point_vector[0],
        point_vector[1],
        euclidian_distance(&point_vector[0], &point_vector[1])
    );
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
fn csv_str_to_point_vector(csv_str: String) -> PointVector {
    let mut reader = csv::Reader::from_reader(csv_str.as_bytes());
    let mut point_vector: PointVector = Vec::new();

    for raw_record in reader.records() {
        let record = match raw_record {
            Ok(raw_record) => raw_record,
            Err(error) => panic!(
                "Error: A problem ocurred while acessing csv records: {:?}",
                error
            ),
        };
        // in future refactor this to consider parsing more data types
        point_vector.push(vec![
            record[0].parse::<f32>().unwrap(),
            record[1].parse::<f32>().unwrap(),
            record[2].parse::<f32>().unwrap(),
            record[3].parse::<f32>().unwrap(),
            record[4].parse::<f32>().unwrap(),
        ]);
    }

    return point_vector;
}

// normalize Points from a PointVector: O(3n) considering entry points not point vector
fn normalize_point_vector(point_vector: &mut PointVector) -> () {
    if point_vector.len() == 0 {
        panic!("Error: PointVector must have items");
    }

    let mut min_point_values: Point = vec![f32::MAX; point_vector[0].len() as usize];
    let mut max_point_values: Point = vec![f32::MIN; point_vector[0].len() as usize];
    let mut max_min_difference = vec![0.0; point_vector[0].len() as usize];

    /*
    println!("{:?}", min_point_values);
    println!("{:?}", max_point_values);
    */

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

    /*
    println!("{:?}", min_point_values);
    println!("{:?}", max_point_values);
    println!("{:?}", max_min_difference);
    */

    // Iterates in points to normalize: O(1n) considering entry points
    for vector_pos in 0..point_vector.len() {
        for point_pos in 0..point_vector[vector_pos].len() {
            point_vector[vector_pos][point_pos] = (point_vector[vector_pos][point_pos]
                - min_point_values[point_pos])
                / max_min_difference[point_pos];
        }
    }
}

// calculates the euclidian distance between two multi dimensional points, the debug part is commented
fn euclidian_distance(p1: &Point, p2: &Point) -> f32 {
    let mut total_square_sum: f32 = 0.0;

    if p1.len() != p2.len() {
        panic!("Error: Different dimensions for two points");
    }

    //print!("sqrt( ");
    for pos in 0..p1.len() {
        total_square_sum += (p1[pos] - p2[pos]).powf(2.0);
        /*
        if pos == 0 {
            print!("( {: ^5.2} - {: ^5.2} )^2", p1[pos], p2[pos])
        } else {
            print!(" + ( {: ^5.2} - {: ^5.2} )^2", p1[pos], p2[pos])
        };
        */
    }
    total_square_sum = total_square_sum.sqrt();

    //println!(" ) = {: ^5.2}", total_square_sum);
    return total_square_sum;
}
