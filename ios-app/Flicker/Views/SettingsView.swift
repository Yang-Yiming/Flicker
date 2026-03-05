import SwiftUI

struct SettingsView: View {
    @ObservedObject var syncService: SyncService
    let storage: StorageService
    @State private var supabaseURL = UserDefaults.standard.string(forKey: "supabase_url") ?? ""
    @State private var anonKey = UserDefaults.standard.string(forKey: "supabase_anon_key") ?? ""
    @State private var speechLocale = UserDefaults.standard.string(forKey: "speech_locale") ?? ""
    @State private var saved = false

    var body: some View {
        Form {
            Section("Speech") {
                Picker("Language", selection: $speechLocale) {
                    Text("Auto (Device Default)").tag("")
                    Text("zh-Hans (简体中文)").tag("zh-Hans")
                    Text("en-US (English)").tag("en-US")
                    Text("ja-JP (日本語)").tag("ja-JP")
                }
                .onChange(of: speechLocale) { _, value in
                    UserDefaults.standard.set(value, forKey: "speech_locale")
                }
            }

            Section("Supabase") {
                TextField("URL", text: $supabaseURL)
                    .textInputAutocapitalization(.never)
                    .autocorrectionDisabled()
                SecureField("Anon Key", text: $anonKey)

                Button("Save") {
                    UserDefaults.standard.set(supabaseURL, forKey: "supabase_url")
                    UserDefaults.standard.set(anonKey, forKey: "supabase_anon_key")
                    saved = true
                    DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) { saved = false }
                }

                if saved {
                    Text("Saved").foregroundColor(.green).font(.caption)
                }
            }

            Section("Sync") {
                Button {
                    Task { await syncService.sync(storage: storage) }
                } label: {
                    HStack {
                        Text("Sync Now")
                        if syncService.isSyncing {
                            Spacer()
                            ProgressView()
                        }
                    }
                }
                .disabled(syncService.isSyncing || !syncService.isConfigured)

                if let error = syncService.syncError {
                    Text(error).foregroundColor(.red).font(.caption)
                }

                if let last = syncService.lastSyncedAt {
                    Text("Last synced: \(last.formatted(date: .abbreviated, time: .shortened))")
                        .font(.caption).foregroundColor(.secondary)
                } else {
                    Text("Never synced").font(.caption).foregroundColor(.secondary)
                }
            }
        }
        .navigationTitle("Settings")
    }
}
