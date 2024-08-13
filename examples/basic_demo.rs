use box2d_sys::*;

fn main() {
    let gravity = b2Vec2 { x: 0.0, y: -10.0 };
    let mut world_def = unsafe { b2DefaultWorldDef() };
    world_def.gravity = gravity;
    let world_id = unsafe { b2CreateWorld(&world_def) };

    let mut body_def = unsafe { b2DefaultBodyDef() };
    body_def.type_ = b2BodyType_b2_dynamicBody;

    let body_id = unsafe { b2CreateBody(world_id, &body_def) };

    let size = b2Vec2 { x: 1.0, y: 1.0 };
    let polygon = unsafe { b2MakeBox(size.x / 2., size.y / 2.) };
    let shape_def = unsafe { b2DefaultShapeDef() };
    unsafe { b2CreatePolygonShape(body_id, &shape_def, &polygon) };

    for i in 0..10 {
        unsafe { b2World_Step(world_id, 1. / 60., 4) };
        println!("Stepped world ({})", i + 1);
    }
}
