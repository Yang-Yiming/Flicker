import Combine
import Foundation

class SyncService: ObservableObject {
    @Published var lastSyncedAt: Date?
    @Published var syncError: String?
    @Published var isSyncing = false

    var isConfigured: Bool {
        let url = UserDefaults.standard.string(forKey: "supabase_url") ?? ""
        let key = UserDefaults.standard.string(forKey: "supabase_anon_key") ?? ""
        return !url.isEmpty && !key.isEmpty
    }

    private var baseURL: String {
        (UserDefaults.standard.string(forKey: "supabase_url") ?? "").trimmingCharacters(in: CharacterSet(charactersIn: "/"))
    }

    private var anonKey: String {
        UserDefaults.standard.string(forKey: "supabase_anon_key") ?? ""
    }

    init() {
        if let ts = UserDefaults.standard.object(forKey: "last_synced_at") as? Date {
            lastSyncedAt = ts
        }
    }

    func sync(storage: StorageService) async {
        guard isConfigured else {
            syncError = "Supabase not configured"
            return
        }

        await MainActor.run { isSyncing = true; syncError = nil }

        do {
            try await pull(storage: storage)
            try await push(storage: storage)
            let now = Date()
            UserDefaults.standard.set(now, forKey: "last_synced_at")
            await MainActor.run {
                lastSyncedAt = now
                isSyncing = false
            }
        } catch {
            await MainActor.run {
                syncError = error.localizedDescription
                isSyncing = false
            }
        }
    }

    // MARK: - Pull

    private func pull(storage: StorageService) async throws {
        var urlString = "\(baseURL)/rest/v1/flickers?select=*"
        if let since = lastSyncedAt {
            urlString += "&updated_at=gt.\(ISO8601DateFormatter().string(from: since))"
        }

        guard let url = URL(string: urlString) else { throw SyncError.badURL }

        var request = URLRequest(url: url)
        request.addValue(anonKey, forHTTPHeaderField: "apikey")
        request.addValue("Bearer \(anonKey)", forHTTPHeaderField: "Authorization")

        let (data, response) = try await URLSession.shared.data(for: request)
        guard let http = response as? HTTPURLResponse, http.statusCode == 200 else {
            throw SyncError.pullFailed
        }

        let rows = try JSONDecoder().decode([FlickerSyncRow].self, from: data)

        await MainActor.run {
            for row in rows {
                guard let remote = row.toFlicker() else { continue }
                let local = storage.flickers.first { $0.id == remote.id }

                let shouldWrite: Bool
                if let local = local {
                    shouldWrite = remote.updatedAt > local.updatedAt
                } else {
                    shouldWrite = true
                }

                if shouldWrite {
                    // Write directly without triggering updatedAt stamp
                    let dir = storage.flickersURL
                    try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
                    try? remote.toFileContent().write(
                        to: dir.appendingPathComponent("\(remote.id).md"),
                        atomically: true, encoding: .utf8
                    )
                }
            }
            storage.load()
        }

    }

    // MARK: - Push

    private func push(storage: StorageService) async throws {
        let allFlickers: [Flicker] = await MainActor.run { storage.allFlickers() }

        let toPush: [Flicker]
        if let since = lastSyncedAt {
            toPush = allFlickers.filter { $0.updatedAt > since }
        } else {
            toPush = allFlickers
        }

        guard !toPush.isEmpty else { return }

        let rows = toPush.map { FlickerSyncRow.from(flicker: $0) }
        let body = try JSONEncoder().encode(rows)

        guard let url = URL(string: "\(baseURL)/rest/v1/flickers") else { throw SyncError.badURL }

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.httpBody = body
        request.addValue(anonKey, forHTTPHeaderField: "apikey")
        request.addValue("Bearer \(anonKey)", forHTTPHeaderField: "Authorization")
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue("resolution=merge-duplicates", forHTTPHeaderField: "Prefer")

        let (_, response) = try await URLSession.shared.data(for: request)
        guard let http = response as? HTTPURLResponse, (200...299).contains(http.statusCode) else {
            throw SyncError.pushFailed
        }

    }
}

// MARK: - Helpers

enum SyncError: LocalizedError {
    case badURL, pullFailed, pushFailed

    var errorDescription: String? {
        switch self {
        case .badURL: return "Invalid Supabase URL"
        case .pullFailed: return "Failed to pull from Supabase"
        case .pushFailed: return "Failed to push to Supabase"
        }
    }
}

private struct FlickerSyncRow: Codable {
    let id: String
    let created_at: String
    let updated_at: String
    let source: String
    let audio_file: String?
    let status: String
    let body: String

    static func from(flicker f: Flicker) -> FlickerSyncRow {
        FlickerSyncRow(
            id: f.id,
            created_at: ISO8601DateFormatter().string(from: f.createdAt),
            updated_at: ISO8601DateFormatter().string(from: f.updatedAt),
            source: f.source,
            audio_file: f.audioFile,
            status: f.status.rawValue,
            body: f.body
        )
    }

    func toFlicker() -> Flicker? {
        let fmt = ISO8601DateFormatter()
        guard let createdAt = fmt.date(from: created_at),
              let updatedAt = fmt.date(from: updated_at),
              let status = FlickerStatus(rawValue: status) else { return nil }

        var f = Flicker(id: id, body: body, audioFile: audio_file)
        f.createdAt = createdAt
        f.updatedAt = updatedAt
        f.source = source
        f.status = status
        return f
    }
}
