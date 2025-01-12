use std::cmp::Ordering;

use tower_lsp::lsp_types::{Position, Range};
use tree_sitter::Point;

pub fn point_to_position(point: Point) -> Position {
    Position {
        line: point.row as u32,
        character: point.column as u32,
    }
}

pub fn range_from_node(node: &tree_sitter::Node) -> Range {
    Range {
        start: point_to_position(node.start_position()),
        end: point_to_position(node.end_position()),
    }
}

pub fn range_contains(range: Range, pos: Position) -> bool {
    range.start <= pos && pos <= range.end
}

pub fn range_extend(range: Range, other: Range) -> Range {
    Range {
        start: pos_min(range.start, other.start),
        end: pos_max(range.end, other.end),
    }
}

pub fn range_for_substring(original_range: Range, original_string: &str, substring: &str) -> Range {
    let offset = original_string.find(substring).unwrap() as u32;
    Range {
        start: Position {
            line: original_range.start.line,
            character: original_range.start.character + offset,
        },
        end: Position {
            line: original_range.start.line,
            character: original_range.start.character + offset + substring.len() as u32,
        },
    }
}

pub fn pos_min(pos: Position, other: Position) -> Position {
    match pos.line.cmp(&other.line) {
        Ordering::Equal => Position {
            line: pos.line,
            character: pos.character.min(other.character),
        },
        Ordering::Less => pos,
        Ordering::Greater => other,
    }
}

pub fn pos_max(pos: Position, other: Position) -> Position {
    match pos.line.cmp(&other.line) {
        Ordering::Equal => Position {
            line: pos.line,
            character: pos.character.max(other.character),
        },
        Ordering::Less => other,
        Ordering::Greater => pos,
    }
}
