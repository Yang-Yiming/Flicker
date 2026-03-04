import Foundation
import Combine

class StorageService: ObservableObject {
    @Published var flickers: [Flicker] = []

    private var baseURL: URL {
        FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
    }

    var flickersURL: URL { baseURL.appendingPathComponent("flickers") }

    func audioURL(for id: String) -> URL {
        baseURL.appendingPathComponent("audio/\(id).m4a")
    }

    func deleteAudio(id: String) throws {
        let url = audioURL(for: id)
        guard FileManager.default.fileExists(atPath: url.path) else { return }
        try FileManager.default.removeItem(at: url)
    }

    func load() {
        let dir = flickersURL
        try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        let files = (try? FileManager.default.contentsOfDirectory(at: dir, includingPropertiesForKeys: nil)) ?? []
        let all = files
            .filter { $0.pathExtension == "md" }
            .compactMap { url -> Flicker? in
                guard let content = try? String(contentsOf: url) else { return nil }
                return Flicker(fileContent: content)
            }
        flickers = all
            .filter { $0.status != .deleted }
            .sorted { $0.createdAt > $1.createdAt }
    }

    /// Returns all flickers including deleted ones (used by sync)
    func allFlickers() -> [Flicker] {
        let dir = flickersURL
        let files = (try? FileManager.default.contentsOfDirectory(at: dir, includingPropertiesForKeys: nil)) ?? []
        return files
            .filter { $0.pathExtension == "md" }
            .compactMap { url -> Flicker? in
                guard let content = try? String(contentsOf: url) else { return nil }
                return Flicker(fileContent: content)
            }
    }

    func save(_ flicker: Flicker) throws {
        var f = flicker
        f.updatedAt = Date()
        let dir = flickersURL
        try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        try f.toFileContent().write(to: dir.appendingPathComponent("\(f.id).md"), atomically: true, encoding: .utf8)
        load()
    }

    func delete(_ flicker: Flicker) throws {
        var f = flicker; f.status = .deleted; try save(f)
    }

    func updateStatus(_ flicker: Flicker, status: FlickerStatus) throws {
        var f = flicker; f.status = status; try save(f)
    }
}
