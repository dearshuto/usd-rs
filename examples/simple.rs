use usd_rs::{LoadState, Path, StreamReader};

fn main() {
    let data = include_str!("cube.usda");
    let mut stream_reader = StreamReader::new(data.as_bytes());
    let mut reader = usd_rs::AsciiReader::new(&mut stream_reader);
    if !reader.read(LoadState::TopLevel) {
        println!("failed to read");
        return;
    }
    if !reader.reconstruct_stage() {
        println!("failed to reconstrude");
        return;
    }

    let Some(stage) =  reader.try_get_stage() else {
            println!("failed to get stage");
            return;
        };

    let path = Path::new("/Cube", "");
    let Some(prim) = stage.find_prim_at_path(&path) else {
        return;
    };

    println!("ChildCount: {}", prim.get_child_count());
    let Some(child) = prim.try_get_child(0) else {
        println!("cannot find the child at 0");
        return;
    };

    let Some(x_form) =  prim.as_x_form() else {
        println!("cannot cast to Xform");
        return;
    };
    let mut matrix_data = Default::default();
    x_form.get_transform(&mut matrix_data);

    println!("LocalMatrix");
    {
        #[rustfmt::skip]
        println!("{}, {}, {}, {}", matrix_data.data[0], matrix_data.data[1], matrix_data.data[2], matrix_data.data[3]);
        #[rustfmt::skip]
        println!("{}, {}, {}, {}",matrix_data.data[4], matrix_data.data[5], matrix_data.data[6], matrix_data.data[7]);
        #[rustfmt::skip]
        println!("{}, {}, {}, {}",matrix_data.data[8], matrix_data.data[9], matrix_data.data[10], matrix_data.data[11]);
    }

    let Some(geom_mesh) = child.as_gemo_mesh() else {
                println!("failed to get GeomMesh");
        return;
    };

    let (x, y, z) = geom_mesh.get_point(0);
    println!("Point0: {}, {}, {}", x, y, z);
}
