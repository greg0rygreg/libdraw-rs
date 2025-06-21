use chrono::{Utc, DateTime, TimeZone};
use libdraw_rs::{Canvas, Color, utils};

fn main() {
    let (w, h) = (27, 3);
    let mut c = Canvas::new(w, h);
    let time = Utc::now().timestamp();
    let naive = DateTime::from_timestamp(time, 0).unwrap();
    let readable = Utc.from_utc_datetime(&naive.naive_utc());

    c.set_time(time);
    c.set_author("gregory greg");

    for i in 0..h-1 { c.set_pixel( 1, i+1, Color::Black); }
    for i in 0..3 { c.set_pixel(i+1, 3, Color::Black); }
    for i in 0..3 { c.set_pixel( 6, i+1, Color::Red); }
    for i in 0..3 { c.set_pixel( i+5, 1, Color::Red); }
    for i in 0..3 { c.set_pixel( i+5, 3, Color::Red); }
    for i in 0..3 { c.set_pixel( 9, i+1, Color::Green); }
    for i in 0..2 { c.set_pixel( 10, i+2, Color::Green); }
    for i in 0..3 { c.set_pixel( 13, i+1, Color::Yellow); }
    for i in 0..3 {
        if i != 1 {
            c.set_pixel(14, i+1, Color::Yellow);
        } else {
            c.set_pixel(15, i+1, Color::Yellow);
        }
    }
    for i in 0..3 { c.set_pixel( 17, i+1, Color::Blue); }
    for i in 0..3 {
        if i != 2 {
            c.set_pixel(18, i+1, Color::Blue);
        } else {
            c.set_pixel(19, i+1, Color::Blue);
        }
    }
    for i in 0..3 { c.set_pixel( 21, i+1, Color::Magenta); }
    for i in 0..3 { c.set_pixel( 23, i+1, Color::Magenta); }
    for i in 0..3 { if i != 2 { c.set_pixel( 22, i+1, Color::Magenta); } }
    for i in 0..3 { c.set_pixel( 25, i+1, Color::Cyan); }
    for i in 0..3 { c.set_pixel( 27, i+1, Color::Cyan); }
    for i in 0..3 { if i != 0 { c.set_pixel( 26, i+1, Color::Cyan); } }

    for y in 0..h {
        for x in 0..w {
            print!("{}", c.format_pixel(x+1, y+1));
        }
        println!("");
    }
    utils::reset();
    println!("\nby: {}\ntime made: {}", c.author, readable.format("%m/%d/%Y @ %H:%M:%S %Z"));
}