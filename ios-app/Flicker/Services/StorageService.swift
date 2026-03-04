import Foundation
import Combine

enum StorageError: Error { case noiCloud }

class StorageService: ObservableObject {
    @Published var flickers: [Flicker] = []

    private var flickersURL: URL? {
        FileManager.default
            .url(forUbiquityContainerIdentifier: "iCloud.com.flicker.app")?
            .appendingPathComponent("Documents/flickers")
    }

    func audioURL(for id: String) -> URL? {
        FileManager.default
            .url(forUbiquityContainerIdentifier: "iCloud.com.flicker.app")?
            .appendingPathComponent("Documents/audio/\(id).m4a")
    }

    func deleteAudio(id: String) throws {
        guard let url = audioURL(for: id),
              FileManager.default.fileExists(atPath: url.path) else { return }
        try FileManager.default.removeItem(at: url)
    }

    func load() {
        guard let dir = flickersURL else { return }
        try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        let files = (try? FileManager.default.contentsOfDirectory(at: dir, includingPropertiesForKeys: nil)) ?? []
        flickers = files
            .filter { $0.pathExtension == "md" && !$0.lastPathComponent.contains(" ") }
            .compactMap { (try? String(contentsOf: $0)).flatMap { Flicker(fileContent: $0) } }
            .filter { $0.status != .deleted }
            .sorted { $0.createdAt > $1.createdAt }
    }

    func save(_ flicker: Flicker) throws {
        guard let dir = flickersURL else { throw StorageError.noiCloud }
        try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        try flicker.toFileContent().write(to: dir.appendingPathComponent("\(flicker.id).md"), atomically: true, encoding: .utf8)
        load()
    }

    func delete(_ flicker: Flicker) throws {
        var f = flicker; f.status = .deleted; try save(f)
    }

    func updateStatus(_ flicker: Flicker, status: FlickerStatus) throws {
        var f = flicker; f.status = status; try save(f)
    }
}
