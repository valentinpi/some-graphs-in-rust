// Note that many of the values are hardcoded and might not work under a
// different resolution. No TODO statement here since this is just for fun.

struct Graph {
    nodes: Vec<(i32, i32)>,
    vertices: Vec<(usize, usize)>,
}

impl std::fmt::Display for Graph {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!("Nodes:");
        for node in &self.nodes {
            print!("({},{}) ", node.0, node.1);
        }
        println!("");
        
        println!("Vertices:");
        for vertex in &self.vertices {
            print!("({},{}) ", vertex.0, vertex.1);
        }
        println!("");

        Ok(())
    }
}

fn random_graph(nodes: i32) -> Graph {
    //let mut graph = Graph {
    //    nodes: vec![(1,-9), (2,7), (9,2), (9,-7)],
    //    vertices: vec![]
    //};

    let mut graph = Graph {
        nodes: vec![],
        vertices: vec![]
    };

    // Insert n nodes on random positions 
    for _i in 0..nodes {
        let x = (rand::random::<u32>() % 21) as i32 - 10;
        let y = (rand::random::<u32>() % 21) as i32 - 10;
        &graph.nodes.push((x, y));
    }
    
    // Add random vertices
    for u in 0..graph.nodes.len() {
        // Since these graphs are undirected, we do not allow for 
        // any duplicate vertices
        for v in u+1..graph.nodes.len() {
            if rand::random() {
                &graph.vertices.push((u, v));
            }
        }
    }

    // You may print the graph if you want since I have implemented
    // the Display trait for it
    //println!("{}", graph);

    graph
}

fn render(sdl_canvas: &mut sdl2::render::WindowCanvas, nodes: i32) {
    sdl_canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    sdl_canvas.clear();

    let graph = random_graph(nodes);

    sdl_canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
    for vertex in &graph.vertices {
        let start = sdl2::rect::Point::new(
            400 + graph.nodes[vertex.0].0 * 40, 400 + graph.nodes[vertex.0].1 * 40);
        let end = sdl2::rect::Point::new(
            400 + graph.nodes[vertex.1].0 * 40, 400 + graph.nodes[vertex.1].1 * 40);
        // Thicker lines
        for i in (-1)..1 {
            for j in (-1)..1 {
                let offset = sdl2::rect::Point::new(i, j);
                let mut _res = sdl_canvas.draw_line(start + offset, end + offset);
            }
        }
    }

    sdl_canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    for node in &graph.nodes {
        let mut _res = sdl_canvas.fill_rect(sdl2::rect::Rect::new(
            400 + node.0 * 40 - 5, 400 + node.1 * 40 - 5, 10, 10));
    }
}

fn main() {
    // Default
    let mut nodes = 5;

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        nodes = args[1].parse::<i32>().unwrap();
        if nodes < 0 {
            println!("Node count can not be negative. Using default value of 5.");
            nodes = 5;
        }
    }

    println!("Drawing graphs with {} nodes", nodes);

    let sdl_context = sdl2::init().unwrap();
    sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "1");
    let sdl_video = sdl_context.video().unwrap();
    let sdl_window = sdl_video.window("Graphs", 800, 800)
        .position_centered()
        .build()
        .unwrap();
    let mut sdl_eventpump = sdl_context.event_pump().unwrap();
    let mut sdl_canvas = sdl_window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let mut running = true;
    let mut redraw = true;
    while running {
        for event in sdl_eventpump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | 
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    running = false;
                },
                sdl2::event::Event::KeyDown { .. } => {
                    redraw = true
                },
                _ => {}
            }
        }

        if redraw {
            render(&mut sdl_canvas, nodes);
            redraw = false;
        }

        sdl_canvas.present();
    }
}
