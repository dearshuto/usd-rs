#[derive(Debug)]
pub enum UpAxis {
    Z,
}

pub enum PropertyType {
    XformOpTranslate([f64; 3]),
    FaceVertexCounts(Vec<i32>),
    FaceVertexIndicies(Vec<i32>),
    Points(Vec<[f32; 3]>),
}
