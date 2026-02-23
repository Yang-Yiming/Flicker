import SwiftUI

struct FlickerDetailView: View {
    @State var flicker: Flicker
    let storage: StorageService
    @State private var editing = false
    @State private var editText = ""
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
}
