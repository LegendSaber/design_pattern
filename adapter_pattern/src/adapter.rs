
trait AdvancedMediaPlayer {
    fn play_vlc(&self, file_name: &str);
    fn play_mp4(&self, file_name: &str);
}


struct VlcPlayer;

impl AdvancedMediaPlayer for VlcPlayer {
    fn play_vlc(&self, file_name: &str) {
        println!("Playing vlc file. Name: {}", file_name);
    }

    fn play_mp4(&self, _file_name: &str) {
        // do nothing
    }
}

struct Mp4Player;

impl AdvancedMediaPlayer for Mp4Player {
    fn play_vlc(&self, _file_name: &str) {
        // do nothing
    }

    fn play_mp4(&self, file_name: &str) {
        println!("Playing mp4 file. Name: {}", file_name);
    }
}

trait MediaPlayer {
    fn play(&self, audio_type: &str, file_name: &str);
}

struct MediaAdapter {
    advanced_media_player: Box<dyn AdvancedMediaPlayer>,
}

impl MediaAdapter {
    fn new(audio_type: &str) -> Self {
        match audio_type {
            "vlc" => MediaAdapter {
                advanced_media_player: Box::new(VlcPlayer),
            },
            "mp4" => MediaAdapter {
                advanced_media_player: Box::new(Mp4Player),
            },
            _ => panic!("Unsupported audio type: {}", audio_type),
        }
    }
}

impl MediaPlayer for MediaAdapter {
    fn play(&self, audio_type: &str, file_name: &str) {
        match audio_type {
            "vlc" => {
                self.advanced_media_player.play_vlc(file_name);
            },
            "mp4" => {
                self.advanced_media_player.play_mp4(file_name);
            },
            _ => println!("Invalid audio type: {}", audio_type),
        }
    }
}

struct AudioPlayer;

impl MediaPlayer for AudioPlayer {
    fn play(&self, audio_type: &str, file_name: &str) {
        match audio_type {
            "mp3" => println!("Playing mp3 file. Name: {}", file_name),
            "vlc" | "mp4" => MediaAdapter::new(audio_type).play(audio_type, file_name),
            _ => println!("Invalid audio type: {}", audio_type),
        }
    }
}

pub(crate) fn test() {
    let audio_player = AudioPlayer;
    audio_player.play("mp3", "beyond the horizon.mp3");
    audio_player.play("mp4", "alone.mp4");
    audio_player.play("vlc", "far far away.vlc");
    audio_player.play("avi", "mind me.avi");
}
