///
///  ColorTensor Container
///
///
///
///
///
///
///
///
///

const std = @import("std");
const dbg = @import("./dbg.zig").dbg;
        
// type definitions
const InnerIndex = usize;
const MemoryAtom = usize;

// constants
const COLOR_VALUE = u8;

pub const Capacity = usize;
pub const Index = Capacity;
pub const Dimension = usize;


const ColorIndex = enum {
    Red,
    Green,
    Blue,
    Alpha,
};
const ColorSlice = struct {
    len: usize,
    index: []const ColorIndex,
    memory: [][*]COLOR_VALUE,
};
const Colors = struct {
    memory: [][*]COLOR_VALUE,

    const Self = @This();

    pub fn init(len: usize, index: []const ColorIndex) !Self {
        const memory = try std.heap.page_allocator.alloc(COLOR_VALUE, len * index.len);
        return Colors{
            .len = len,
            .index = index,
            .memory = memory,
        };
    }

    // getter
    // pub fn get(idx: Index, ci: ColorIndex) [*]ColorSlice{

    // };
};

pub const IndexError = error{
    OutOfBounds,
};

inline fn mul(xs: []const Dimension) Index {
    var result: Index = 1;
    for (xs) |x| result *= x;
    return result;
}

const tt = std.testing;
test "initialize and retrieve color values" {
}