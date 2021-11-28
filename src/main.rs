use std::fs::File;
// use std::error::Error;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, Sink};


// struct AudioPacket {
//     sample_rate: u32,
//     data: Vec<i16>
// }

// fn audio_processing(filename: String) -> Result<AudioPacket, Box<dyn Error>> {
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let file = BufReader::new(File::open(filename).unwrap());
//     let source = Decoder::new(file).unwrap();
//     let sample_rate = source.sample_rate();
//     let samples = source.buffered();
//     let data: Vec<i16> = samples.clone().collect();
//     stream_handle.play_raw(samples.convert_samples())?;
//     Ok(
//         AudioPacket {
//             sample_rate: sample_rate,
//             data: data
//         }
//     )
// }


// fn main() {


//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let sink = Sink::try_new(&stream_handle).unwrap();
//     let file = BufReader::new(File::open("..\\lofi\\Monma - Calm Lands-doxeMNXYFqk.mp3").unwrap());
//     let source = Decoder::new(file).unwrap();
//     let samples = source.buffered();
//     // Add a dummy source of the sake of the example.
//     sink.append(samples);
//     // The sound plays in a separate thread. This call will block the current thread until the sink
//     // has finished playing all its queued sounds.
//     // sink.sleep_until_end();
//     std::thread::sleep(std::time::Duration::from_secs(100));
// }





// --ICED--
use std::{env, fs, path};
use std::sync::{Arc, Mutex};
use iced::{button, Sandbox, Element, Column, Align};
use iced::{Text, widget::image::Image, Row, Button, Length};
use iced::{Settings, window};

pub fn main() -> iced::Result {
    Player::run(Settings {
        window: window::Settings {
            size: (500, 200),
            decorations: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
    
}

#[derive(Default)]
struct Song {
    name: String,
    path: String

}

fn add_song_to_sink(sink: &Sink, path: String) {
    sink.stop();
    sink.pause();
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source.buffered().clone());
    sink.play();
}


struct Player {
    playing: bool,
    song: usize,
    songs: Vec<Song>,
    last_song: button::State,
    next_song: button::State,
    play_pause: button::State,
    sink: Sink
}

impl std::default::Default for Player {
    fn default() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        return Player {
            playing: false,
            song: 0,
            songs: Vec::new(),
            last_song: button::State::default(),
            next_song: button::State::default(),
            play_pause: button::State::default(),
            sink: sink
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    PlayPausePressed,
    BackPressed,
    FrontPressed
}

impl Sandbox for Player {
    type Message = Message;

    fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            panic!("You Suck!");
        }
        let song_dir = args[1].clone();
        let mut songs: Vec<Song> = Vec::new();
        for file in fs::read_dir(path::Path::new(&song_dir)).unwrap() {
            let metadata: fs::DirEntry = file.unwrap();
            if metadata.metadata().unwrap().is_file() {
                songs.push(Song {
                    name: metadata.path().file_name().unwrap().to_str().unwrap().to_owned(),
                    path: metadata.path().to_str().unwrap().to_owned()
                });
                println!("{:?}", &songs[songs.len() - 1].path);
            }
        }
        Self {
            playing: false,
            songs: songs,
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Music Player")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::BackPressed => {
                if self.song == 0 {
                    self.song = self.songs.len() - 1;
                } else {
                    self.song -= 1;
                }

                add_song_to_sink(&self.sink, self.songs[self.song].path.clone());
            }
            Message::FrontPressed => {
                if self.song == self.songs.len() - 1 {
                    self.song = 0;
                } else {
                    self.song += 1;
                }
                add_song_to_sink(&self.sink, self.songs[self.song].path.clone());
            }
            Message::PlayPausePressed => {
                self.playing = !self.playing;
                if self.playing == true {
                    if self.sink.empty() {
                        add_song_to_sink(&self.sink, self.songs[self.song].path.clone());
                    } else {
                        self.sink.play();
                    }
                } else {
                   self.sink.pause();
                }
            }
        }

        println!("{:?}", &message);
        println!("Sink Empty : {:?}", self.sink.empty());
        println!("Sink Paused : {:?}", self.sink.is_paused());
        println!("Sink Length : {:?}", self.sink.len());
        println!("Sink Volume : {:?}", self.sink.volume());
    }

    fn view(&mut self) -> Element<Message> {
        let play_pause_image: &str;
        let mut song_name = &"".to_owned();
        if self.playing {
            play_pause_image = "./assets/images/pause.png";
        } else {
            play_pause_image = "./assets/images/play.png";
        }
        if self.songs.len() != 0 {
            song_name = &self.songs[self.song].name;
        }

        let button_size = 1;
        let spacing = 90;

        Column::new()
            .align_items(Align::Center).padding(20u16)
            // .push(Text::new("Music Player").size(50))
            .push(Text::new(song_name).size(20))
            .spacing(20)
            .push(
                Row::new().width(Length::Fill)
                .push(
                    Button::new(&mut self.last_song, Image::new("./assets/images/previous.png").width(Length::Fill).height(Length::Fill)).on_press(Message::BackPressed).width(Length::FillPortion(button_size)).style(style::Button::Used)
                ).spacing(spacing)
                .push(
                    Button::new(&mut self.play_pause, Image::new(play_pause_image).width(Length::Fill).height(Length::Fill)).on_press(Message::PlayPausePressed).width(Length::FillPortion(button_size)).style(style::Button::Used)
                ).spacing(spacing)
                .push(
                    Button::new(&mut self.next_song, Image::new("./assets/images/next.png").width(Length::Fill).height(Length::Fill)).on_press(Message::FrontPressed).width(Length::FillPortion(button_size)).style(style::Button::Used)
                ).spacing(spacing)
            )
        .into()
    }
}


mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Used
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Used => button::Style {
                    background: Some(Background::Color(Color::from_rgba(
                        1.0, 1.0, 1.0, 1.0
                    ))),
                    border_radius: 5.0,
                    
                    shadow_offset: Vector::new(1.0,1.0),
                    ..button::Style::default()
                },
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                background: Some(Background::Color(Color::from_rgba(
                    16.0/255.0, 93.0/255.0, 176.0/255.0, 1.0
                ))),
                ..active
            }
        }
    }
}