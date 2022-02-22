#!/usr/bin/env swift
import Foundation

func getInput() throws -> [String:[String]] {
    let fileURL = URL(fileURLWithPath: "input.txt")
    let fileContents = try String(contentsOf: fileURL, encoding: .utf8)
    var edges: [String:[String]] = [:]
    
    fileContents.enumerateLines { (line, _) in
        let contents = line.split(separator: "-")
        if edges[String(contents[0])] != nil {
            edges[String(contents[0])]?.append(String(contents[1]))
        } else {
            edges[String(contents[0])] = [String(contents[1])]
        }
        if edges[String(contents[1])] != nil {
            edges[String(contents[1])]?.append(String(contents[0]))
        } else {
            edges[String(contents[1])] = [String(contents[0])]
        }
    }
    return edges
}

extension String {
    /// Returns true if all characters are lowercased, false otherwise
    func isLowercased() -> Bool {
        var isLowercased = true;
        self.forEach { char in
            if !char.isLowercase {
                isLowercased = false;
            }
        }
        return isLowercased
    }

    func isUppercased() -> Bool {
        !self.isLowercased()
    }
}

extension Array where Element == String {
    func containsAmount(of str: String) -> Int {
        var count = 0
        for string in self {
            if str == String(string) {
                count += 1
            }
        }
        return count
    } 
}

func get_routes_part1(edges: [String: [String]]) -> [[String]] {
    var all_paths: [[String]] = []
    var paths = [["start"]]
    while !paths.isEmpty {
        let path = paths.popLast()!

        if path.last! == "end" {
            all_paths.append(path)
            continue
        }

        for candidate: String in edges[path.last!]! {
            if !candidate.isLowercased() || !path.contains(candidate) {
                var newPath = path
                newPath.append(candidate)
                paths.append(newPath)
            } 
        }
    }
    return all_paths
}


let edges = try getInput()
let paths = get_routes_part1(edges: edges)
let count = paths.count
print("Routes part 1:", count)