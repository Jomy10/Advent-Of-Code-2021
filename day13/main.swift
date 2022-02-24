#!/usr/bin/env swift
// This is probably the ugliest and most inefficient code I have ever written
// I sould have used a dictionary instead of just arrays containing every single dot
// But oh well, that's a challenge for another day
import Foundation

extension Array {
    func split(x: Int) -> ([Element], [Element]) {
        let left = self[0..<x]
        let right = self[x+1..<self.count]
        return (Array(left), Array(right))
    }
}

extension Array: CustomStringConvertible where Element == [Character] {
    var description: String {
        var s = ""
        for row in self {
            s += "\n[ "
            for element in row {
                s += String(element as Character)
                s += " "
            }
            s += "]"
        }
        return s
    }
}

extension Array where Element == [Character] {
    /// Splits an array at the given coordinate
    func split(y: Int) -> ([[Character]], [[Character]]) {
        var top: [[Character]]  = []
        var bottom: [[Character]] = []
        for row in self {
            top.insert(row[0..<y].map { char in char }, at: top.count)
            bottom.insert(row[y+1..<row.count].map { char in char }, at: bottom.count)
        }
        return (top, bottom)
    }

    // Flips a single array
    func flipUp() -> [Element] {
        var flipped = self
        (0..<self.count).forEach { index in
            flipped[index] = self[index].reversed()
        }
        return flipped
    }


    /// Folds an array at the given coordinate
    func foldUp(at y: Int) -> [Element] {
        let split = self.split(y: y)
        var flipped: [Element] = split.1.flipUp() // [Element]()
        var toIndex = 0
        var merged = split.0
        
        let startAt = split.0.count - flipped.count
        var index = 0
        // print("startAt:", startAt)
        // print("n: h   w")
        // print("0:", split.0.count, split.0[0].count)
        // print("1:", flipped.count, flipped[0].count)
        (startAt..<split.0.count).forEach { indexInMerged in
            var newArr: [Character] = []
            for i in 0..<Swift.min(split.0[indexInMerged].count, flipped[index].count) {
                // print(indexInMerged, i, index)
                if split.0[indexInMerged][i] == "#" || flipped[index][i] == "#" {
                    newArr.insert("#", at: i)
                } else {
                    newArr.insert(".", at: i)
                }
            }
            index += 1

            merged[indexInMerged] = newArr
        }
        return merged
    }

    func split(x: Int) -> ([[Character]], [[Character]]) {
        var left: [[Character]] = (0..<x).map { i in (0..<self[0].count).map { j in "." as Character} }
        var right: [[Character]] = (0..<self.count-x-1).map { i in (0..<self[0].count).map { j in "." as Character } }
        (0..<self[0].count).forEach { index in
            for i in 0..<self.count {
                if i < x {
                    left[i][index] = self[i][index]
                } else if i > x {
                    right[i-x-1][index] = self[i][index]
                }
            }
        }
        return (left, right)
    }

    func flipLeft() -> [Element] {
        Array(self.reversed())
    }

    func foldLeft(at x: Int) -> [Element] {
        let (left, right) = self.split(x: x)
        let flippedRight = right.flipLeft()
        var merged = left
        let startAt = left.count - right.count
        var index = 0
        (startAt..<left.count).forEach { indexInMerged in 
            var newArr: [Character] = []
            for i in 0..<left[indexInMerged].count {
                if left[indexInMerged][i] == "#" || flippedRight[index][i] == "#" {
                    newArr.insert("#", at: i)
                } else {
                    newArr.insert(".", at: i)
                }
            }
            index += 1

            merged[indexInMerged] = newArr
        }
        return merged
    }
}

extension Collection where Self.Iterator.Element: RandomAccessCollection {
    func transposed() -> [[Self.Iterator.Element.Iterator.Element]] {
        guard let firstRow = self.first else { return [] }
        return firstRow.indices.map { index in 
            self.map{ $0[index] }
        }
    }
}

struct Coordinate {
    var x: Int
    var y: Int
}

struct FoldInstruction {
    var alongAxis: Axis
    var position: Int
}

enum Axis {
    case X
    case Y
}

func getInput(file: String) throws -> ([Coordinate], [FoldInstruction]) {
    let url = URL(fileURLWithPath: file)
    let contents = try String(contentsOf: url, encoding: .utf8)
    var coordinates: [Coordinate] = []
    var foldInstructions: [FoldInstruction] = []
    contents.enumerateLines { line, _ in
        if line.contains("fold") {
            var instructions = line
            instructions.removeSubrange(line.range(of: "fold along ")!)
            let sep = instructions.split(separator: "=")
            var axis: Axis
            if sep[0] == "x" { axis = Axis.X } else { axis = Axis.Y }
            foldInstructions.append(FoldInstruction(alongAxis: axis, position: Int(sep[1])!))
        } else if line.contains(",") {
            let coords = line.trimmingCharacters(in: CharacterSet.whitespacesAndNewlines).split(separator: ",")
            let coordinate = Coordinate(x: Int(coords[0])!, y: Int(coords[1])!)
            coordinates.append(coordinate)
        } 
    }
    return (coordinates, foldInstructions)
}

// Get input
let (coordinates, foldInstructions) = try getInput(file: "input.txt")
var maxX: Int = 0
var maxY: Int = 0

coordinates.forEach { coordinate in
    if coordinate.x > maxX { maxX = coordinate.x } 
    if coordinate.y > maxY { maxY = coordinate.y }
}
print(maxX, maxY)

var grid: [[Character]] = Array(repeating: Array(repeating: ".", count: maxY + 1), count: maxX + 1)
for coordinate in coordinates {
    grid[coordinate.x][coordinate.y] = "#"
}

let firstInstruction = foldInstructions.first!
let gridAfterFirstInstruction: [[Character]]
switch firstInstruction.alongAxis {
    case Axis.X:
        gridAfterFirstInstruction = grid.foldLeft(at: firstInstruction.position)
    case Axis.Y:
        gridAfterFirstInstruction = grid.foldUp(at: firstInstruction.position)
}
var dotsCount = 0
for row in gridAfterFirstInstruction {
    for element in row {
        if element == "#" {
            dotsCount += 1
        }
    }
}

print("Visible dots after first fold:", dotsCount)

for instr in foldInstructions {
    print(instr)
    print(grid.count, grid[0].count)
    switch instr.alongAxis {
        case Axis.X:
            grid = grid.foldLeft(at: instr.position)
        case Axis.Y:
            grid = grid.foldUp(at: instr.position)
    }
}

print(grid.transposed().description)

// for instr in foldInstructions {
//     grid = fold(alongAxis: instr.alongAxis, at: instr.position, grid)
// }

// print(grid)
