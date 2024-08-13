pub use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to join the main voice channel, why: {0}")]
    FailedToJoinMainVoiceChannel(JoinError),
    #[error("Not in the main voice channel to retrieve Call")]
    NotInVoiceChannelToRetrieveCall,
    #[error("Unknown youtube video error: {0}")]
    UnknownVideoError(VideoError),
    #[error("Youtube video not found")]
    YoutubeVideoNotFound,
}

pub async fn join_main_voice_channel(bot: &impl AsBot) -> Result<()> {
    let songbird = bot.get_songbird_manager();

    songbird
        .join(bot.get_main_guild_id(), bot.get_main_voice_channel_id())
        .await
        .map_err(Error::FailedToJoinMainVoiceChannel)?;

    Ok(())
}

pub async fn play_youtube_video(bot: &impl AsBot, url_or_id: impl AsRef<str>) -> Result<()> {
    let songbird_manager = bot.get_songbird_manager();

    let call_handler_arc = songbird_manager.get(bot.get_main_guild_id()).ok_or(Error::NotInVoiceChannelToRetrieveCall)?;

    let video_options = VideoOptions { quality: VideoQuality::HighestAudio, filter: VideoSearchOptions::Audio, ..Default::default() };

    let video = match Video::new_with_options(url_or_id.as_ref(), video_options.clone()) {
            Ok(video) => video,
            Err(VideoError::VideoNotFound) => Err(Error::YoutubeVideoNotFound)?,
            Err(e) => Err(Error::UnknownVideoError(e))?,
    };
    let info = video.get_info().await.map_err(Error::UnknownVideoError)?;

    let mut format = rusty_ytdl::choose_format(&info.formats, &video_options).map_err(Error::UnknownVideoError)?;
    let source = AudioHttpRequest::new(bot.get_reqwest_client(), mem::take(&mut format.url));

    let mut call_handler = call_handler_arc.lock().await;
    call_handler.stop();

    call_handler.play_input(source.into());

    Ok(())
}
