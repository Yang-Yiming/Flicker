import Speech
import AVFoundation

class SpeechService: ObservableObject {
    @Published var transcript = ""
    @Published var isRecording = false

    private let recognizer = SFSpeechRecognizer()
    private var audioEngine = AVAudioEngine()
    private var request: SFSpeechAudioBufferRecognitionRequest?
    private var task: SFSpeechRecognitionTask?
    private var recorder: AVAudioRecorder?

    func requestPermission(completion: @escaping (Bool) -> Void) {
        SFSpeechRecognizer.requestAuthorization { status in
            DispatchQueue.main.async { completion(status == .authorized) }
        }
    }

    func startRecording(audioURL: URL) throws {
        let session = AVAudioSession.sharedInstance()
        try session.setCategory(.record, mode: .measurement, options: .duckOthers)
        try session.setActive(true, options: .notifyOthersOnDeactivation)

        recorder = try AVAudioRecorder(url: audioURL, settings: [
            AVFormatIDKey: kAudioFormatMPEG4AAC,
            AVSampleRateKey: 44100,
            AVNumberOfChannelsKey: 1,
            AVEncoderAudioQualityKey: AVAudioQuality.medium.rawValue
        ])
        recorder?.record()

        request = SFSpeechAudioBufferRecognitionRequest()
        guard let request else { return }
        request.shouldReportPartialResults = true

        let node = audioEngine.inputNode
        node.installTap(onBus: 0, bufferSize: 1024, format: node.outputFormat(forBus: 0)) { [weak self] buffer, _ in
            request.append(buffer)
        }
        audioEngine.prepare()
        try audioEngine.start()
        isRecording = true

        task = recognizer?.recognitionTask(with: request) { [weak self] result, _ in
            if let t = result?.bestTranscription.formattedString {
                DispatchQueue.main.async { self?.transcript = t }
            }
        }
    }

    func stopRecording() {
        audioEngine.stop()
        audioEngine.inputNode.removeTap(onBus: 0)
        audioEngine = AVAudioEngine()
        request?.endAudio()
        task?.finish()
        recorder?.stop()
        recorder = nil
        try? AVAudioSession.sharedInstance().setActive(false)
        isRecording = false
    }
}
