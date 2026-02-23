import Foundation

enum FlickerStatus: String, CaseIterable {
    case inbox, kept, archived, deleted
}

struct Flicker: Identifiable {
    var id: String
    var createdAt: Date
    var source: String
    var audioFile: String?
    var status: FlickerStatus
    var body: String

    init?(fileContent: String) {
        let lines = fileContent.components(separatedBy: "\n")
        guard lines.first == "---" else { return nil }

        var fmLines: [String] = []
        var bodyLines: [String] = []
        var inFM = true

        for line in lines.dropFirst() {
            if inFM && line == "---" { inFM = false; continue }
            if inFM { fmLines.append(line) } else { bodyLines.append(line) }
        }
        guard !inFM else { return nil }

        var fm: [String: String] = [:]
        for line in fmLines {
            let parts = line.split(separator: ":", maxSplits: 1).map { $0.trimmingCharacters(in: .whitespaces) }
            if parts.count == 2 { fm[parts[0]] = parts[1] }
        }

        guard let id = fm["id"],
              let statusStr = fm["status"],
              let status = FlickerStatus(rawValue: statusStr) else { return nil }

        self.id = id
        self.status = status
        self.source = fm["source"] ?? "ios"
        self.audioFile = fm["audio_file"]
        self.body = bodyLines.joined(separator: "\n").trimmingCharacters(in: .whitespacesAndNewlines)
        self.createdAt = fm["created_at"].flatMap { ISO8601DateFormatter().date(from: $0) } ?? Date()
    }

    init(id: String, body: String) {
        self.id = id
        self.createdAt = Date()
        self.source = "ios"
        self.audioFile = nil
        self.status = .inbox
        self.body = body
    }

    func toFileContent() -> String {
        var s = "---\nid: \(id)\ncreated_at: \(ISO8601DateFormatter().string(from: createdAt))\nsource: \(source)\n"
        if let audio = audioFile { s += "audio_file: \(audio)\n" }
        s += "status: \(status.rawValue)\n---\n\n\(body)\n"
        return s
    }
}
