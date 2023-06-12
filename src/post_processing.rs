use crate::configuration::{ ClusterGroup, PointVector };
use plotters::prelude::*;
use std::path::Path;

#[derive(Clone)]
struct CartesianCoord {
    coordinate_vector: Vec<(f32, f32)>, // defines k clusters
    color: RGBColor, // move database points to here
}
type CartesianCoordVector = Vec<CartesianCoord>;

// 5 sequence of 2 colors and red at end, first color of sequences for core and second for border, red to outliers
const DEFINED_COLORS: [plotters::style::RGBColor; 11] = [
    RGBColor(0, 0, 255),
    RGBColor(0, 0, 150),
    RGBColor(0, 255, 0),
    RGBColor(0, 150, 0),
    RGBColor(255, 255, 0),
    RGBColor(200, 200, 0),
    RGBColor(0, 255, 255),
    RGBColor(0, 200, 200),
    RGBColor(255, 0, 255),
    RGBColor(200, 0, 200),
    RGBColor(255, 0, 0),
];

// parses a cluster_group to plot a 2d cartesian view
pub fn parse_and_plot_cartesion_2d(cluster_group: &ClusterGroup, database: &PointVector, att_1_pos: usize, att_2_pos: usize, att_1_name: &str, att_2_name: &str, out_file_name: &str) -> () {
    
    if cluster_group.clusters.len() == 0 {
        panic!("Error: cluster_group is invalid");
    }

    let mut cartesian_vector: CartesianCoordVector = Vec::new();
    let mut cartesian_pos: i8 = -1;
    let mut color_sequence: usize = 0;

    // creates cartesion_2d to params attributes
    for cluster_pos in 0..cluster_group.clusters.len(){
        // core indexes
        if cluster_group.clusters[cluster_pos].core_indexes.len() > 0 {
            cartesian_vector.push( CartesianCoord {
                coordinate_vector: Vec::new(),
                color: DEFINED_COLORS[color_sequence * 2usize],
            });
            cartesian_pos += 1;
            for core_pos in 0..cluster_group.clusters[cluster_pos].core_indexes.len() {
                let database_point = database[cluster_group.clusters[cluster_pos].core_indexes[core_pos]].clone();
                cartesian_vector[cartesian_pos as usize].coordinate_vector.push( (database_point[att_1_pos], database_point[att_2_pos]) );
            }
        }
        // border indexes
        if cluster_group.clusters[cluster_pos].border_indexes.len() > 0 {
            cartesian_vector.push( CartesianCoord {
                coordinate_vector: Vec::new(),
                color: DEFINED_COLORS[(color_sequence * 2usize) + 1usize],
            });
            cartesian_pos += 1;
            for border_pos in 0..cluster_group.clusters[cluster_pos].border_indexes.len() {
                let database_point = database[cluster_group.clusters[cluster_pos].border_indexes[border_pos]].clone();
                cartesian_vector[cartesian_pos as usize].coordinate_vector.push( (database_point[att_1_pos], database_point[att_2_pos]) );
            }
        }
        // rotate in color sequence, limit of 5 sequences of 2 color representing core and border points
        color_sequence = (color_sequence + 1) % ((DEFINED_COLORS.len() / 2) as usize);
    }
    // outlier indexes
    if cluster_group.outlier_indexes.len() > 0 {
        cartesian_vector.push( CartesianCoord {
            coordinate_vector: Vec::new(),
            color: DEFINED_COLORS[DEFINED_COLORS.len()-1], // red color is the last
        });
        cartesian_pos += 1;
        for outlier_pos in 0..cluster_group.outlier_indexes.len() {
            let database_point = database[cluster_group.outlier_indexes[outlier_pos]].clone();
            cartesian_vector[cartesian_pos as usize].coordinate_vector.push( (database_point[att_1_pos], database_point[att_2_pos]) );
        }
    }

    let _ = plot_cartesion_2d(&cartesian_vector, att_1_name, att_2_name, out_file_name);
}

// plots a clusters 2d cartesian, only 2 attributes per image but multiple clusters
fn plot_cartesion_2d(cartesian_vector: &CartesianCoordVector, axis_x_label: &str, axis_y_label: &str, out_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {

    // creates working path for windows and linux and converts to str
    let output_path = Path::new(".").join("postprocessing").join(out_file_name);
    let output_path_str = match output_path.to_str() {
        None => panic!("Error: Database_path is not a valid UTF-8 sequence"),
        Some(s) => s,
    };

    // creates a bit map object and sets its destination, size and white fill
    let image = BitMapBackend::new(output_path_str, (1024, 768)).into_drawing_area();
    image.fill(&WHITE)?;

    // divide image in areas to plot and maintain borders
    let areas = image.split_by_breakpoints([944], [80]);

    // builds the cartesian drawing except by circles
    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc(axis_x_label)
        .y_desc(axis_y_label)
        .draw()?;

    // foreach cartesian_vector plots circles
    for cartesian_pos in 0..cartesian_vector.len(){
        scatter_ctx.draw_series(
            cartesian_vector[cartesian_pos].coordinate_vector
                .iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, cartesian_vector[cartesian_pos].color.filled())),
        )?;
    }

    // Expect for IO errors
    image.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("{} has been saved!", out_file_name);
    Ok(())
}