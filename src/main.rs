mod mazes;

fn main() {
    env_logger::init();

    // let maze = mazes::Maze::new(625, 345).unwrap();
    let maze = mazes::Maze::new(301, 301).unwrap();
    // let maze = mazes::Maze::new(11, 11).unwrap();
    
    let image = maze.build_image();
    image.save("output.png").unwrap();
}

