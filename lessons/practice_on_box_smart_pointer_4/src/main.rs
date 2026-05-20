struct AudioSample;
struct ImageFile;

trait Media {}

impl Media for AudioSample {}
impl Media for ImageFile {}

fn main() {
    let audio_1 = AudioSample;
    let audio_2 = Box::new(AudioSample);

    let audio_3 = audio_1;
    let audio_4 = audio_2;

    let image_1 = Box::new(ImageFile);

    let media_collection: Vec<Box<dyn Media>> = vec![Box::new(audio_3), audio_4, image_1]; // Fix this line
}
