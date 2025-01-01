#![allow(non_snake_case)]
use helium::{
    app::{events::EventQueue, view::View, App}, hstack, vstack, widgets::{
        icon::feather_icons, Container, Rect, Spacer, Text, Widget,
    }, AxisAlignment, Color, LayoutSolver, Size, BLACK, WHITE
};
use std::{
    env,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

const BACKGROUND: Color = Color::Hex("#121212");
const GREY: Color = Color::Hex("#414141");
const SPOTIFY_GREEN: Color = Color::Hex("#3be477");

// TODO theres some sizes that are making the icons pixelated, very weird
fn main() {
    env::set_var("RUST_LOG", "trace,wgpu_core=error,naga=warn,wgpu_hal=error,async_std=warn");
    env_logger::init();

    let event_queue = EventQueue::new();

    let announcements = Rect::new(0.0, 400.0, BACKGROUND)
        .flex_width(1)
        .corner_radius(24);

    let chips = hstack! {
        Chip("Playlist"),
        Chip("Album"),
        Chip("Artist"),
        Chip("Downloaded")
    }
    .spacing(12)
    .fill_width();

    let sidebar = vstack! {
        hstack!{
            feather_icons::menu(),
            Text::new("Your library").color(WHITE),
			Spacer(),
            feather_icons::plus(),
            feather_icons::arrow_right()
        }
		.cross_axis_alignment(AxisAlignment::Center)
		.fill_width(),
        chips,
        hstack!{
            feather_icons::search(),
            Spacer(),
            Text::new("Recents").color(WHITE),
            feather_icons::list(),
        }.fill_width(),
        SidebarPlaylist("Liked songs"),
        SidebarPlaylist("Channel Orange"),
        SidebarPlaylist("Wunna"),
        SidebarPlaylist("2014 Forest Hills Drive")
    }
    .spacing(24)
    .padding(24)
    .fill_height();

    let mainpanel = vstack! {
        announcements,
        hstack!{
            Chip("All"),
            Chip("Music"),
            Chip("Podcasts")
        }.spacing(12),
		hstack!{
			HomePlaylist("Car music"),
			HomePlaylist("Euphoria"),
			HomePlaylist("Liked songs"),
			HomePlaylist("Hype hype"),
		}
		.spacing(24)
		.fill_width(),
		hstack!{
			HomePlaylist("Car music"),
			HomePlaylist("Euphoria"),
			HomePlaylist("Liked songs"),
			HomePlaylist("Hype hype"),
		}
		.spacing(24)
		.fill_width(),
		HomeSection(),
		HomeSection(),
		HomeSection(),
		HomeSection(),
		HomeSection(),
    }
    .padding(24)
    .spacing(24)
    .fill_width();

    let home_page = hstack!{sidebar,mainpanel}.fill_width().fill_height();

    let home = vstack! {
        Navbar(),
        home_page,
        BottomBar()
    }
	.color(BACKGROUND)
    .fill_height()
    .fill_width();

    let (_, mut layout) = home.build();
    LayoutSolver::solve(&mut *layout, Size::new(500.0, 500.0));

    // TODO im probably going to be using this a lot so probs just make it a function
    // maybe LayoutSolver::solve_and_write(path:&str)
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open("C:/Users/wakun/Projects/Tools/Rust-UI/examples/temp/layout.txt")
        .unwrap();
    let mut writer = BufWriter::new(file);

    writer.write(format!("{:#?}", layout).as_bytes()).unwrap();
    writer.flush().unwrap();

    let home = View::new(home, event_queue);
    App::new().add_view(home).run();
}

fn DiscoveryPlaylist() -> impl Widget{
	vstack!{
		Rect::new(200.0, 200.0, BACKGROUND).corner_radius(12),
		Text::new("Daily mix")
	}
	.spacing(12)
}

fn HomeSection() -> impl Widget{
	vstack!{
		hstack!{
			Text::new("Made for charlemagne")
			.font_size(24)
			.color(WHITE),
			Spacer(),
			Text::new("Show all").color(WHITE),
		}
		.fill_width(),
		hstack!{
			DiscoveryPlaylist(),
			DiscoveryPlaylist(),
			DiscoveryPlaylist(),
			DiscoveryPlaylist(),
			DiscoveryPlaylist(),
			DiscoveryPlaylist(),
		}.spacing(36)
		
	}
	.spacing(10)
	.fill_width()
}

fn BottomBar() -> impl Widget {
    hstack! {
        hstack!{
            Rect::new(50.0, 50.0, BLACK)
            .corner_radius(12),
            vstack!{
                Text::new("You've been missed")
				.color(WHITE),
                Text::new("PARTYNEXTDOOR")
				.color(WHITE)
            }
			.spacing(4)
        }
		.spacing(12)
		.cross_axis_alignment(AxisAlignment::Center),
        Spacer(),
        vstack!{
            hstack!{
                feather_icons::shuffle(),
                feather_icons::skip_back(),
                feather_icons::play(),
                feather_icons::skip_forward(),
                feather_icons::repeat()
            }
			.spacing(16),
            hstack!{
                Text::new("0:00")
				.color(WHITE),
                Rect::new(350.0, 5.0, BLACK).corner_radius(2),
                Text::new("4:00")
				.color(WHITE)
            }
			.spacing(8)
			.cross_axis_alignment(AxisAlignment::Center)
        }
		.spacing(12)
		.fit_height()
		.cross_axis_alignment(AxisAlignment::Center),
		Spacer(),
    }
    .fill_width()
	.padding(16)
}

fn Navbar() -> impl Widget {
    hstack! {
		feather_icons::more_horizontal(),
		feather_icons::chevron_left(),
		feather_icons::chevron_right(),
		Spacer(),
		feather_icons::home(),
		feather_icons::search(),
		Text::new("What do you want to play?")
		.color(WHITE),
		Spacer(),
		feather_icons::bell(),
		feather_icons::users()
    }
    .fill_width()
	.spacing(12)
	.padding(12)
	.cross_axis_alignment(AxisAlignment::Center)
}

fn Chip(text: &str) -> impl Widget {
    let text = Text::new(text).color(WHITE);

    Container::new(text)
    .corner_radius(14)
    .color(GREY)
    .padding(12)
}

fn SidebarPlaylist(title: &str) -> impl Widget {
    hstack! {
        Rect::new(50.0, 50.0, BACKGROUND).corner_radius(12),
        vstack!{
            Text::new(title).color(WHITE),
            hstack!{
                Text::new("Playlist").color(WHITE),
                Text::new("Charlemagne").color(WHITE)
            }
			.spacing(12)
        }
    }
    .spacing(12)
	.cross_axis_alignment(AxisAlignment::Center)
}

fn HomePlaylist(name:&str) -> impl Widget{
	hstack! {
		Rect::new(50.0, 50.0, BACKGROUND).corner_radius(12),
		Text::new(name).color(WHITE)
	}
	.spacing(12)
	.cross_axis_alignment(AxisAlignment::Center)
	.fill_width()
}
