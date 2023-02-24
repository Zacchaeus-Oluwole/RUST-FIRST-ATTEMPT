use opencv::{
    Result,
    prelude::*,
    videoio,
    highgui
};

fn main() -> Result<()>{
    let mut cam = videoio::VideoCapture::new(0,videoio::CAP_ANY)?;
    highgui::named_window("Window",highgui::WINDOW_FULLSCREEN)?;
    let mut frame = Mat::default();
    loop{
        cam.read(&mut frame)?;
        highgui::imshow("Window", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113{
            break;
        }
    }

    Ok(())
}
