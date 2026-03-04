import SwiftUI

struct NewFlickerView: View {
    let storage: StorageService
    @State private var text = ""
    @StateObject private var speech = SpeechService()
    @State private var hasAudio = false
    @State private var flickerID = String(UUID().uuidString.replacingOccurrences(of: "-", with: "").prefix(8).lowercased())
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                TextEditor(text: $text)
                    .padding()
                Button { toggleRecording() } label: {
                    Image(systemName: speech.isRecording ? "stop.circle.fill" : "mic.circle.fill")
                        .font(.system(size: 52))
                        .foregroundStyle(speech.isRecording ? .red : .accentColor)
                }
                .padding(.bottom, 24)
            }
            .navigationTitle("New Flicker")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        if hasAudio { try? storage.deleteAudio(id: flickerID) }
                        dismiss()
                    }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Save") {
                        let audioFile = hasAudio ? "audio/\(flickerID).m4a" : nil
                        try? storage.save(Flicker(id: flickerID, body: text, audioFile: audioFile))
                        dismiss()
                    }
                    .disabled(text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                }
            }
            .onChange(of: speech.transcript) { _, new in text = new }
            .alert("Recording Error", isPresented: .init(
                get: { speech.errorMessage != nil },
                set: { if !$0 { speech.errorMessage = nil } }
            )) {
                Button("OK") { speech.errorMessage = nil }
            } message: {
                Text(speech.errorMessage ?? "")
            }
        }
    }

    private func toggleRecording() {
        if speech.isRecording {
            speech.stopRecording()
        } else {
            speech.requestPermission { granted in
                guard granted else {
                    speech.errorMessage = "Microphone or speech recognition permission denied."
                    return
                }
                let url = storage.audioURL(for: flickerID)
                do {
                    try FileManager.default.createDirectory(
                        at: url.deletingLastPathComponent(),
                        withIntermediateDirectories: true
                    )
                    try speech.startRecording(audioURL: url)
                    hasAudio = true
                } catch {
                    speech.errorMessage = error.localizedDescription
                }
            }
        }
    }
}
