import SwiftUI

struct FlickerListView: View {
    @StateObject private var storage = StorageService()
    @StateObject private var syncService = SyncService()
    @State private var selectedStatus: FlickerStatus? = nil
    @State private var showingNew = false
    @State private var showingSettings = false
    @State private var isQuickRecording = false

    var filtered: [Flicker] {
        guard let s = selectedStatus else { return storage.flickers }
        return storage.flickers.filter { $0.status == s }
    }

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack {
                        FilterChip(label: "All", selected: selectedStatus == nil) { selectedStatus = nil }
                        ForEach(FlickerStatus.allCases.filter { $0 != .deleted }, id: \.self) { s in
                            FilterChip(label: s.rawValue.capitalized, selected: selectedStatus == s) { selectedStatus = s }
                        }
                    }.padding(.horizontal)
                }
                .padding(.vertical, 8)

                List(filtered) { flicker in
                    NavigationLink(destination: FlickerDetailView(flicker: flicker, storage: storage)) {
                        FlickerRow(flicker: flicker)
                    }
                }
                .safeAreaInset(edge: .bottom) { Color.clear.frame(height: 80) }
            }
            .overlay(alignment: .bottom) {
                QuickRecordView(storage: storage, isRecording: $isQuickRecording)
            }
            .navigationTitle("Flicker")
            .toolbar {
                ToolbarItem(placement: .topBarLeading) {
                    Button { showingSettings = true } label: { Image(systemName: "gearshape") }
                }
                ToolbarItem(placement: .topBarTrailing) {
                    Button { showingNew = true } label: { Image(systemName: "plus") }
                        .disabled(isQuickRecording)
                }
            }
            .sheet(isPresented: $showingNew) { NewFlickerView(storage: storage) }
            .sheet(isPresented: $showingSettings) {
                NavigationStack {
                    SettingsView(syncService: syncService, storage: storage)
                        .toolbar {
                            ToolbarItem(placement: .confirmationAction) {
                                Button("Done") { showingSettings = false }
                            }
                        }
                }
            }
            .onAppear {
                storage.load()
                if syncService.isConfigured {
                    Task { await syncService.sync(storage: storage) }
                }
            }
        }
    }
}

struct FilterChip: View {
    let label: String
    let selected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(label)
                .font(.subheadline)
                .padding(.horizontal, 12).padding(.vertical, 6)
                .background(selected ? Color.accentColor : Color(.systemGray5))
                .foregroundColor(selected ? .white : .primary)
                .clipShape(Capsule())
        }
    }
}

struct FlickerRow: View {
    let flicker: Flicker

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(flicker.body.isEmpty ? "(empty)" : flicker.body).lineLimit(2)
            HStack {
                Text(flicker.id).font(.caption2).foregroundColor(.secondary)
                Spacer()
                Text(flicker.status.rawValue).font(.caption2).foregroundColor(.secondary)
            }
        }
        .padding(.vertical, 2)
    }
}
