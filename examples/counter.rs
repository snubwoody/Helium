use helium::{
    app::{events::EventQueue, view::View, App}, 
	hstack, 
	widgets::Rect, 
	AxisAlignment,
	BLACK, BLUE, GREEN
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let event_loop = EventQueue::new();

	// TODO add align center,left,etc
	let main = hstack!{
		Rect::new(200.0,200.0,BLACK),
		Rect::new(200.0,200.0,BLUE),
		Rect::new(200.0,200.0,GREEN),
	}
	.spacing(12)
	.fill()
	.cross_axis_alignment(AxisAlignment::Center)
	.main_axis_alignment(AxisAlignment::Center);

	let page = View::new(main,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}

