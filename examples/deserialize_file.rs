fn main() {
    let usda_file = include_str!("simple_cube.usda");
    let usd = usd_rs::serializer::from_str(usda_file).unwrap();
    for definition in usd.definitions() {
        println!("name: {}", definition.name);
        for property in &definition.properties {
            match &property.property {
                usd_rs::serializer::PropertyType::XformOpTranslate(_) => todo!(),
                usd_rs::serializer::PropertyType::FaceVertexCounts(vertex_count) => {
                    println!("faceVertexCount = {:?}", vertex_count);
                }
                usd_rs::serializer::PropertyType::FaceVertexIndicies(face_vertex_indices) => {
                    println!("faceVertexIndices = {:?}", face_vertex_indices);
                }
                usd_rs::serializer::PropertyType::Points(points) => {
                    println!("points = {:?}", points);
                }
                _ => {}
            }
        }
    }
}
