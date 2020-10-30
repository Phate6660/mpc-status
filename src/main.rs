use mpd::{Client, Song, State, Status};
use std::fmt;

struct PlayState {
    sta: State,
}

impl fmt::Display for PlayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.sta)
    }
}

fn format_time(time: i64) -> String {
    let minutes = (time / 60) % 60;
    let seconds = time % 60;

    format!("{:0>2}:{:0>2}", minutes, seconds)
}

fn main() {
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    let status: Status = c.status().unwrap();
    let song: Song = c.currentsong().unwrap().unwrap();
    let na = "N/A".to_string();
    let art = song.tags.get("Artist").unwrap_or(&na);
    let alb = song.tags.get("Album").unwrap_or(&na);
    let tit = song.title.as_ref().unwrap();
    let elap = status.elapsed.unwrap().num_seconds();
    let elapsed = format_time(elap);
    let dur = status.duration.unwrap().num_seconds();
    let duration = format_time(dur);
    let stat = status.state;
    let state = PlayState { sta: stat };
    println!("{} - {} - {} -- [{}] -- [{}/{}]", art, alb, tit, state, elapsed, duration);
}
