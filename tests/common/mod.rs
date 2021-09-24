use crate::MyWorld;

pub fn assert_api_error(world: MyWorld, msg: &str) -> MyWorld {
    match world {
        MyWorld::ApiError(e) => {
            assert_eq!(e.to_string(), msg)
        }
        _ => panic!("Invalid world state"),
    }
    MyWorld::Nothing
}
