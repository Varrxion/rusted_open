use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::{BufReader, Read};

pub struct AudioManager {
    sounds: RwLock<HashMap<String, Vec<u8>>>,  // Store audio data in memory
    audio_queue: RwLock<VecDeque<String>>,     // Queue for sounds to be played
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl AudioManager {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().expect("Failed to create audio stream");
        AudioManager {
            sounds: RwLock::new(HashMap::new()),
            audio_queue: RwLock::new(VecDeque::new()),
            _stream: stream,
            stream_handle,
        }
    }

    // Enqueue a sound for playback
    pub fn enqueue_sound(&self, name: &str) {
        let mut queue = self.audio_queue.write().unwrap();
        queue.push_back(name.to_string());
    }

    // Process and play sounds in the queue
    pub fn process_audio_queue(&self) -> Result<(), String> {
        let mut queue = self.audio_queue.write().unwrap();

        while let Some(name) = queue.pop_front() {
            // Play each sound from the queue
            self.play_sound(&name)?;
        }

        Ok(())
    }

    // Play the sound
    pub fn play_sound(&self, name: &str) -> Result<(), String> {
        let sounds = self.sounds.read().unwrap();
        let sound_data = sounds.get(name).ok_or("Sound not found".to_string())?;
        let cursor = std::io::Cursor::new(sound_data.clone());
        let source = Decoder::new(BufReader::new(cursor)).map_err(|_| "Failed to decode audio".to_string())?;

        let sink = Sink::try_new(&self.stream_handle).map_err(|_| "Failed to create audio sink".to_string())?;
        sink.append(source);
        sink.detach(); // Let it play independently
        
        Ok(())
    }

    // Load sounds from a directory
    pub fn load_sound(&self, name: &str, path: &str) -> Result<(), String> {
        let mut sounds = self.sounds.write().unwrap();
        
        if sounds.contains_key(name) {
            return Ok(()); // Sound is already loaded
        }

        let file = File::open(path).map_err(|_| "Failed to open audio file".to_string())?;
        let mut buffer = Vec::new();
        BufReader::new(file).read_to_end(&mut buffer).map_err(|_| "Failed to read audio file".to_string())?;
        
        sounds.insert(name.to_string(), buffer);
        Ok(())
    }

    // Load sounds from directory
    pub fn load_sounds_from_directory(&self, dir_path: &str) -> Result<(), String> {
        let paths = std::fs::read_dir(dir_path).map_err(|_| "Failed to read directory".to_string())?;

        for path in paths {
            let entry = path.map_err(|_| "Failed to read directory entry".to_string())?;
            let file_name = entry.file_name().into_string().map_err(|_| "Invalid file name".to_string())?;
            let full_path = entry.path();

            if full_path.is_file() {
                if let Some(extension) = full_path.extension() {
                    if extension == "mp3" || extension == "wav" || extension == "flac" {
                        let name = file_name.trim_end_matches(".mp3").trim_end_matches(".wav").trim_end_matches(".flac");
                        self.load_sound(name, full_path.to_str().unwrap()).map_err(|e| format!("Error loading sound '{}': {}", name, e))?;
                    }
                }
            }
        }

        Ok(())
    }
}
