#!/usr/bin/env swift
import Foundation

func getInput(file: String) throws -> [[Character]] {
    let url = URL(fileURLWithPath: file)
    let contents = try String(contentsOf: url, encoding: .utf8)

    return [[]]
}


print(try getInput(file: "input.txt"))