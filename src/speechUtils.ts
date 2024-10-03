function speak(s: string) {
	const synth = window.speechSynthesis;
	const utterance = new SpeechSynthesisUtterance(s);
	synth.speak(utterance);
}
