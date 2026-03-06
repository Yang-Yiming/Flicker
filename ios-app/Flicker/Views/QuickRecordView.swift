import SwiftUI

struct QuickRecordView: View {
    let storage: StorageService
    @Binding var isRecording: Bool

    @StateObject private var speech = SpeechService()
    @State private var flickerID = ""
    @State private var hasAudio = false
    @State private var isActive = false

    var body: some View {
        VStack(spacing: 12) {
            if isActive {
                transcriptPanel
                    .transition(.move(edge: .bottom).combined(with: .opacity))
            }

            recordButton
        }
        .padding(.bottom, 16)
        .animation(.easeInOut(duration: 0.25), value: isActive)
        .onDisappear {
            if isActive { stopAndSave() }
        }
    }

    // MARK: - Subviews

    private var transcriptPanel: some View {
        HStack(alignment: .top, spacing: 8) {
            Circle()
                .fill(.red)
                .frame(width: 8, height: 8)
                .opacity(isActive ? 1 : 0)
                .scaleEffect(isActive ? 1.0 : 0.5)
                .animation(.easeInOut(duration: 0.8).repeatForever(autoreverses: true), value: isActive)
                .padding(.top, 6)

            Text(speech.transcript.isEmpty ? "Listening…" : speech.transcript)
                .font(.subheadline)
                .foregroundStyle(speech.transcript.isEmpty ? .secondary : .primary)
                .frame(maxWidth: .infinity, alignment: .leading)
                .lineLimit(4)
        }
        .padding(12)
        .background(.ultraThinMaterial, in: RoundedRectangle(cornerRadius: 12))
        .padding(.horizontal, 16)
    }

    private var recordButton: some View {
        Button { isActive ? stopAndSave() : startQuickRecord() } label: {
            Image(systemName: isActive ? "stop.circle.fill" : "mic.circle.fill")
                .font(.system(size: 64))
                .foregroundStyle(isActive ? .white : .red)
                .background {
                    if isActive {
                        Circle().fill(.red).frame(width: 64, height: 64)
                    }
                }
                .shadow(color: .black.opacity(0.15), radius: 4, y: 2)
        }
    }

    // MARK: - Actions

    private func startQuickRecord() {
        flickerID = String(UUID().uuidString.replacingOccurrences(of: "-", with: "").prefix(8).lowercased())
        speech.transcript = ""
        hasAudio = false

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
                isActive = true
                isRecording = true
            } catch {
                speech.errorMessage = error.localizedDescription
            }
        }
    }

    private func stopAndSave() {
        speech.stopRecording()
        isActive = false
        isRecording = false

        let text = speech.transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        if text.isEmpty {
            // No transcript — clean up audio file
            try? storage.deleteAudio(id: flickerID)
        } else {
            let audioFile = hasAudio ? "audio/\(flickerID).m4a" : nil
            try? storage.save(Flicker(id: flickerID, body: text, audioFile: audioFile))
        }
    }
}
