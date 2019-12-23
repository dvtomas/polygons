use polygons::io;
use polygons::structures::Edge;
use polygons::structures::Point;
use polygons::stuff;

fn floats_are_same(f1: f64, f2: f64) -> bool {
    let d = f1 - f2;
    return d.abs() < std::f64::EPSILON;
}

#[test]
fn rectangle() {
    let mut polygons: Vec<Vec<Edge>> = Vec::new();

    let points: Vec<Point> = io::read_vector("tests/rectangle.txt");
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for p in points.iter() {
        xs.push(p.x);
        ys.push(p.y);
    }

    let num_points = xs.len();
    let polygon = stuff::create_polygon(num_points, &xs, 0.0, &ys, 0.0, 0);
    polygons.push(polygon);

    let mut nodes = Vec::new();
    for p in polygons.iter() {
        // group edges to nodes, 4 at the time
        nodes.append(&mut stuff::group_edges(4, p.clone()));
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = stuff::group_nodes(4, nodes);
    }

    let pxs: [f64; 2] = [0.6, 0.5];
    let pys: [f64; 2] = [0.6, -0.5];

    let mut distances: [f64; 2] = [0.0; 2];
    stuff::get_distances_edge(&nodes, 2, &pxs, &pys, &mut distances);
    assert_eq!(distances, [0.4, 0.5]);

    distances = [0.0; 2];
    stuff::get_distances_vertex(&nodes, 2, &pxs, &pys, &mut distances);
    assert!(floats_are_same(distances[0], 0.5656854249492381));
    assert!(floats_are_same(distances[1], 0.7071067811865476));

    let mut indices: [usize; 2] = [0; 2];
    stuff::get_closest_vertices(&nodes, 2, &pxs, &pys, &mut indices);
    assert_eq!(indices, [2, 0]);

    let mut contains: [bool; 2] = [false; 2];
    stuff::contains_points(&nodes, 2, &pxs, &pys, &mut contains);
    assert_eq!(contains, [true, false]);
}

#[test]
fn polygon() {
    let mut polygons: Vec<Vec<Edge>> = Vec::new();

    let points: Vec<Point> = io::read_vector("tests/polygon.txt");
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for p in points.iter() {
        xs.push(p.x);
        ys.push(p.y);
    }

    let num_points = xs.len();
    let polygon = stuff::create_polygon(num_points, &xs, 0.0, &ys, 0.0, 0);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 5.0, &ys, 0.0, num_points);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 10.0, &ys, 0.0, 2 * num_points);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 15.0, &ys, 0.0, 3 * num_points);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 20.0, &ys, 0.0, 4 * num_points);
    polygons.push(polygon);

    let mut nodes = Vec::new();
    for p in polygons.iter() {
        // group edges to nodes, 4 at the time
        nodes.append(&mut stuff::group_edges(4, p.clone()));
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = stuff::group_nodes(4, nodes);
    }

    const NUM_REFERENCE_POINTS: usize = 5000;

    let reference_points: Vec<Point> = io::read_vector("tests/reference/reference_points.txt");
    let mut pxs = Vec::new();
    let mut pys = Vec::new();
    for p in reference_points.iter() {
        pxs.push(p.x);
        pys.push(p.y);
    }

    let mut distances: [f64; NUM_REFERENCE_POINTS] = [0.0; NUM_REFERENCE_POINTS];
    stuff::get_distances_edge(&nodes, NUM_REFERENCE_POINTS, &pxs, &pys, &mut distances);
    let reference_distances = io::read_vector("tests/reference/distances_edge.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let mut distances = [0.0; NUM_REFERENCE_POINTS];
    stuff::get_distances_vertex(&nodes, NUM_REFERENCE_POINTS, &pxs, &pys, &mut distances);
    let reference_distances = io::read_vector("tests/reference/distances_vertex.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let mut indices: [usize; NUM_REFERENCE_POINTS] = [0; NUM_REFERENCE_POINTS];
    stuff::get_closest_vertices(&nodes, NUM_REFERENCE_POINTS, &pxs, &pys, &mut indices);
    let reference_indices = io::read_vector("tests/reference/closest_indices.txt");
    for (&x, &rx) in indices.iter().zip(reference_indices.iter()) {
        assert_eq!(x, rx);
    }

    let mut contains: [bool; NUM_REFERENCE_POINTS] = [false; NUM_REFERENCE_POINTS];
    stuff::contains_points(&nodes, NUM_REFERENCE_POINTS, &pxs, &pys, &mut contains);
    let reference_bools = io::read_vector("tests/reference/contains_points.txt");
    for (&x, &rx) in contains.iter().zip(reference_bools.iter()) {
        assert_eq!(x, rx);
    }
}
