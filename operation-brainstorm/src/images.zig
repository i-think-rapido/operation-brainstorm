//const log = @import("./log.zig").log;
const std = @import("std");
const rl = @import("raylib");

pub const VoxelColorsError = error {
    OutOfBounds,
};

pub const VoxelColors = struct {

    dimX: usize,
    dimY: usize,
    dimZ: usize,
    colors: []rl.Color,

    const Self = @This();

    pub fn init(dimX: usize, dimY: usize, dimZ: usize) !Self {
        const allocator = std.heap.page_allocator;
        const slice = try allocator.alloc(rl.Color, @intCast(dimX * dimY * dimZ));
        return Self{ .dimX = dimX, .dimY = dimY, .dimZ = dimZ, .colors = slice };
    }
    pub fn deinit(self: *Self) void {
        const allocator = std.heap.page_allocator;
        allocator.free(self.colors);
    }

    pub fn size(self: *const Self) usize {
        return self.dimX * self.dimY * self.dimZ;
    }

    pub fn get(self: *const Self, idx: usize) rl.Color {
        return self.colors[@intCast(idx)];
    }
    pub fn set(self: *Self, idx: usize, color: rl.Color) void {
        self.colors[idx] = color;
    }
    pub fn index(self: *const Self, x: usize, y: usize, z: usize) VoxelColorsError!usize {
        if (x < 0 or x >= self.dimX or y < 0 or y >= self.dimY or z < 0 or z >= self.dimZ) {
            return VoxelColorsError.OutOfBounds; // Return max usize for out of bounds
        }
        return @intCast((z * self.dimY + y) * self.dimX + x);
    }

    pub fn fromFile(filename: [:0]const u8) !Self {
        const image = try rl.loadImage(filename);
        const imageWidth = image.width;
        const imageHeight = image.height;
        const layers = 1;

        var colors = try VoxelColors.init(@intCast(imageWidth), @intCast(imageHeight), @intCast(layers));

        for (0..@intCast(imageWidth)) |i| {
            for (0..@intCast(imageHeight)) |j| {
                for (0..@intCast(layers)) |k| {
                    colors.set(try colors.index(i, j, k), rl.getImageColor(image, @intCast(i), @intCast(j)));
                }
            }
        }

        return colors;
    }

    pub fn array(self: *const Self) ![]u8 {
        const colorCount = self.dimX * self.dimY * self.dimZ * 4;
        const allocator = std.heap.page_allocator;
        const slice = try allocator.alloc(u8, colorCount);
        for (0..self.dimX * self.dimY * self.dimZ) |i| {
            const color = self.colors[i];
            slice[i * 4 + 0] = @intCast(color.r);
            slice[i * 4 + 1] = @intCast(color.g);
            slice[i * 4 + 2] = @intCast(color.b);
            slice[i * 4 + 3] = @intCast(color.a);
        }
        return slice;
    }
    pub fn dearray(arr: []u8) void {
        const allocator = std.heap.page_allocator;
        allocator.free(arr);
    }
    
};




