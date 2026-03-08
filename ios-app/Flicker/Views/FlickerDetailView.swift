import SwiftUI

struct FlickerDetailView: View {
    @State var flicker: Flicker
    let storage: StorageService
    @State private var editing = false
    @State private var editText = ""
    @State private var copiedToClipboard = false
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 16) {
                if editing {
                    TextEditor(text: $editText)
                        .frame(minHeight: 200)
                        .padding(4)
                        .overlay(RoundedRectangle(cornerRadius: 8).stroke(Color.accentColor))
                } else {
                    Text(flicker.body.isEmpty ? "(empty)" : flicker.body)
                        .frame(maxWidth: .infinity, alignment: .leading)
                }

                Divider()

                Group {
                    LabeledContent("ID", value: flicker.id)
                    LabeledContent("Status", value: flicker.status.rawValue.capitalized)
                    LabeledContent("Source", value: flicker.source)
                    LabeledContent("Created", value: flicker.createdAt.formatted(date: .abbreviated, time: .shortened))
                }
                .font(.caption)
                .foregroundColor(.secondary)

                HStack {
                    Menu("Status") {
                        ForEach(FlickerStatus.allCases.filter { $0 != .deleted }, id: \.self) { s in
                            Button(s.rawValue.capitalized) {
                                try? storage.updateStatus(flicker, status: s)
                                flicker.status = s
                            }
                        }
                    }.buttonStyle(.bordered)

                    Menu {
                        Button("ChatGPT") { copyAndOpen(scheme: "chatgpt://") }
                        Button("Claude") { copyAndOpen(scheme: "claude://") }
                        Button("Copy Only") { copyAndOpen(scheme: nil) }
                    } label: {
                        Label(copiedToClipboard ? "Copied!" : "Chat", systemImage: copiedToClipboard ? "checkmark" : "ellipsis.bubble")
                    }
                    .buttonStyle(.bordered)

                    Spacer()
                    Button("Delete", role: .destructive) {
                        try? storage.delete(flicker)
                        dismiss()
                    }.buttonStyle(.bordered)
                }
            }
            .padding()
        }
        .navigationTitle("Flicker")
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            if editing {
                Button("Save") { flicker.body = editText; try? storage.save(flicker); editing = false }
            } else {
                Button("Edit") { editText = flicker.body; editing = true }
            }
        }
    }

    private func copyAndOpen(scheme: String?) {
        let template = UserDefaults.standard.string(forKey: "chat_prompt_template")
            .flatMap { $0.isEmpty ? nil : $0 } ?? Flicker.defaultChatPromptTemplate
        let prompt = template.replacingOccurrences(of: "{{content}}", with: flicker.body)
        UIPasteboard.general.string = prompt
        copiedToClipboard = true
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) { copiedToClipboard = false }
        if let scheme, let url = URL(string: scheme) {
            UIApplication.shared.open(url)
        }
    }
}
