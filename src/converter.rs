use genmesh::{generators::Cube, Quad, MapToVertices, Vertices};
use obj;
use libfj::{robocraft, robocraft_simple::FactoryAPI};

pub fn robot_by_id_to_3d(id: usize) -> Result<obj::Obj, ()> {
    let api = FactoryAPI::new();
    let result = api.get(id);
    if let Ok(robot) = result {
        return Ok(robot_to_wavefront(robot.response.into()));
    } else {
        println!("Error getting robot: {}", result.err().unwrap());
    }
    Err(())
}

/// Convert a Robocraft robot to a 3D model in Wavefront OBJ format.
pub fn robot_to_wavefront(robot: robocraft::Cubes) -> obj::Obj {
    let mut wavefront_obj = obj::Obj{
        data: obj::ObjData {
            position: vec![],
            texture: vec![[0.0, 0.0]],
            normal: vec![],
            objects: vec![],
            material_libs: vec![],
        },
        path: std::path::PathBuf::new(),
    };

    for cube in robot.into_iter() {
        convert_cube(&mut wavefront_obj, cube);
    }
    wavefront_obj
}

fn convert_cube(object: &mut obj::Obj, cube: &robocraft::Cube) {
    match cube.id {
        0 => println!("How did you get here?"),
        _ => default_convert(object, cube),
    }
}

fn default_convert(object: &mut obj::Obj, cube: &robocraft::Cube) {
    let mut last = object.data.position.len();
    object.data.position.extend::<Vec::<[f32; 3]>>(
        Cube::new().vertex(|v|
            [(v.pos.x * 0.5) + (cube.x as f32), (v.pos.y * 0.5) + (cube.y as f32), (v.pos.z * 0.5) + (cube.z as f32)])
        .vertices()
        .collect()
    );
    object.data.normal.extend::<Vec::<[f32; 3]>>(
        Cube::new().vertex(|v|
            [(v.normal.x * 0.5) + (cube.x as f32), (v.normal.y * 0.5) + (cube.y as f32), (v.normal.z * 0.5) + (cube.z as f32)])
        .vertices()
        .collect()
    );
    let polys = Cube::new().vertex(|_| {last+=1; return last-1;})
        .map(|Quad{x: v0, y: v1, z: v2, w: v3}|
            obj::SimplePolygon(vec![
            obj::IndexTuple(v0, Some(0), Some(v0)),
            obj::IndexTuple(v1, Some(0), Some(v1)),
            obj::IndexTuple(v2, Some(0), Some(v2)),
            obj::IndexTuple(v3, Some(0), Some(v3))
            ])
            /*obj::SimplePolygon(vec![
            obj::IndexTuple(v0, None, None),
            obj::IndexTuple(v1, None, None),
            obj::IndexTuple(v2, None, None),
            obj::IndexTuple(v3, None, None)
            ])*/
        ).collect();
    let objects_len = object.data.objects.len();
    object.data.objects.push(
        obj::Object{
            name: format!("Cube-ID{}-NUM{}", cube.id, objects_len),
            groups: vec![
                obj::Group {
                    name: format!("Cube-ID{}-NUM{}-0", cube.id, objects_len),
                    index: 0,
                    material: None,
                    polys: polys
                },
            ]
        }
    );
}
