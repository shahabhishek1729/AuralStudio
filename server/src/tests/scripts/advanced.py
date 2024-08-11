import pydub.AudioSegment as AudioSegment

audio_file = AudioSegment.from_mp3("example.mp3")
louder_audio_file = audio_file + 18
louder_audio_file.export("example_louder.mp3", "mp3")
