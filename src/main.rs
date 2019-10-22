extern crate winit;

fn main() 
{
    use winit::EventsLoop;
    use winit::Window;
    use winit::Event;
    use winit::WindowEvent;
    use winit::ControlFlow;

    let mut events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();
    window.set_title("Window");

    events_loop.run_forever(|event|
     {
        match event
        {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } =>
            {
                println!("Window exit");
                ControlFlow::Break
            },
            Event::WindowEvent { event: WindowEvent::Refresh, .. } =>
            {
                println!("Refresh");
                ControlFlow::Continue               
            },           
            _ =>
            {
                
                ControlFlow::Continue                    
            },
        }
    });
    
}