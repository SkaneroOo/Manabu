use iced::widget::{button, column, progress_bar, row, text, Column};
use iced::{Element, Font};

use super::utils::ColoredText;
use super::{Manabu, Message};

impl Manabu {
    pub fn view_no_data(&self) -> Element<Message> {
        let dl = self.downloads.get(0).unwrap();

        row![
            column![
                ColoredText::new(60.0 * self.settings.text_scale).push("Required data not found", 255, 0, 0).finalize(),
                text("Please, download required data").size(30.0 * self.settings.text_scale),
                dl.view(),
            ].align_x(iced::Alignment::Center),
        ].align_y(iced::Alignment::Center).into()
    }
}

use iced::futures::{SinkExt, Stream, StreamExt};
use iced::stream::try_channel;
use iced::Subscription;

use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

// Just a little utility function
pub fn file<I: 'static + Hash + Copy + Send + Sync, T: ToString>(
    id: I,
    url: T,
) -> iced::Subscription<(I, Result<Progress, Error>)> {
    Subscription::run_with_id(
        id,
        download(url.to_string()).map(move |progress| (id, progress)),
    )
}

fn download(url: String) -> impl Stream<Item = Result<Progress, Error>> {
    try_channel(1, move |mut output| async move {
        let app_path = dirs::config_local_dir().unwrap().join("manabu");
        if !Path::new(&app_path).exists() {
            std::fs::create_dir(&app_path).unwrap();
        }
        let kanji_path = app_path.join("kanji.json");

        let mut file = File::create(&kanji_path).unwrap();

        let response = reqwest::get(&url).await?;
        // let total = response.content_length().ok_or(Error::NoContentLength)?;
        let total = 3218155;
        
        let _ = output.send(Progress::Downloading { percent: 0.0 }).await;
        
        let mut byte_stream = response.bytes_stream();
        let mut downloaded = 0;
        
        while let Some(next_bytes) = byte_stream.next().await {
            let bytes = next_bytes?;
            downloaded += bytes.len();
            
            file.write(&bytes).unwrap();

            let _ = output
                .send(Progress::Downloading {
                    percent: 100.0 * downloaded as f32 / total as f32,
                })
                .await;
        }

        let _ = output.send(Progress::Finished).await;

        Ok(())
    })
}

#[derive(Debug, Clone)]
pub enum Progress {
    Downloading { percent: f32 },
    Finished,
}

#[derive(Debug, Clone)]
pub enum Error {
    RequestFailed(Arc<reqwest::Error>),
    NoContentLength,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::RequestFailed(Arc::new(error))
    }
}

#[derive(Debug)]
pub struct Download {
    pub id: usize,
    pub state: State,
}

#[derive(Debug, PartialEq)]
pub enum State {
    Idle,
    Downloading { progress: f32 },
    Finished,
    Errored,
}

impl Download {
    pub fn new(id: usize) -> Self {
        Download {
            id,
            state: State::Idle,
        }
    }

    pub fn start(&mut self) {
        match self.state {
            State::Idle { .. }
            | State::Finished { .. }
            | State::Errored { .. } => {
                self.state = State::Downloading { progress: 0.0 };
            }
            State::Downloading { .. } => {}
        }
    }

    pub fn progress(
        &mut self,
        new_progress: Result<Progress, Error>,
    ) {
        if let State::Downloading { progress } = &mut self.state {
            match new_progress {
                Ok(Progress::Downloading { percent }) => {
                    *progress = percent;
                }
                Ok(Progress::Finished) => {
                    self.state = State::Finished;
                }
                Err(_error) => {
                    println!("Error: {:?}", _error);
                    self.state = State::Errored;
                }
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Downloading { .. } => {
                file(self.id, "https://res.cloudinary.com/do5vqgzdn/raw/upload/v1727979614/kanji.json")
                    .map(Message::DownloadProgressed)
            }
            _ => Subscription::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let current_progress = match &self.state {
            State::Idle { .. } => 0.0,
            State::Downloading { progress } => *progress,
            State::Finished { .. } => 100.0,
            State::Errored { .. } => 0.0,
        };

        let progress_bar = progress_bar(0.0..=100.0, current_progress);

        let control: Element<_> = match &self.state {
            State::Idle => button("Start the download!")
                .on_press(Message::Download(self.id))
                .into(),
            State::Finished => {
                column!["Download finished!"]
                    .spacing(10)
                    .align_x(iced::Alignment::Center)
                    .into()
            }
            State::Downloading { .. } => {
                text!("Downloading... {current_progress:.2}%").into()
            }
            State::Errored => column![
                "Something went wrong :(",
                button("Try again").on_press(Message::Download(self.id)),
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center)
            .into(),
        };

        Column::new()
            .spacing(10)
            .padding(10)
            .align_x(iced::Alignment::Center)
            .push(progress_bar)
            .push(control)
            .into()
    }
}