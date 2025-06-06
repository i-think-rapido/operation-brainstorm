//const log = @import("./log.zig").log;
const std = @import("std");
const rl = @import("raylib");
const vc = @cImport({
    @cInclude("voxel-color.h");
});
const dbg = @import("./dbg.zig").dbg;



pub const VoxelColor = struct {
    voxel_color: vc.VoxelColors,

    const Self = @This();

    pub fn init(x: i32, y: i32, z: i32) !VoxelColor {
        const voxel_color = vc.new_voxel_color(x, y, z);
        if (voxel_color == null) {
            return error.OutOfMemory;
        }
        return VoxelColor{
            .voxel_color = voxel_color,
        };
    }

    pub fn color(self: *Self, index: i32) rl.Color {
        return rl.Color(
            vc.voxel_color_r(self.voxel_color, index, color.r),
            vc.voxel_color_g(self.voxel_color, index, color.g),
            vc.voxel_color_b(self.voxel_color, index, color.b),
            vc.voxel_color_a(self.voxel_color, index, color.a),
        );
    }

    pub fn setColor(self: *Self, index: i32, c: rl.Color) void {
        vc.set_voxel_color_r(self.voxel_color, index, c.r);
        vc.set_voxel_color_g(self.voxel_color, index, c.g);
        vc.set_voxel_color_b(self.voxel_color, index, c.b);
        vc.set_voxel_color_a(self.voxel_color, index, c.a);
    }
};






// pub const ImageList = struct {
//     list: std.ArrayList(Image),

//     const Self = @This();

//     pub fn init(capacity: usize) !ImageList {
//         return ImageList{
//             .list = try 
//                 std.ArrayList(Image)
//                     .initCapacity(std.heap.page_allocator, capacity)
//         };
//     }
//     pub fn deinit(self: Self) void {
//         for (self.list.items) |item| item.deinit();
//         self.list.deinit();
//     }

//     pub fn push(self: *Self, image: Image) !void {
//         try self.list.append(image);
//     }

//     pub fn get(self: *const Self, idx: usize) !?Image {
//         if (idx >= self.list.items[0..].len) {
//             return null;
//         }
//         return self.list.items[idx];
//     }

//     pub fn height(self: *const Self) i32 {
//         return self.list.getLastOrNull().?.height();
//     }
//     pub fn width(self: *const Self) i32 {
//         return self.list.getLastOrNull().?.width();
//     }
//     pub fn len(self: Self) usize {
//         return self.list.capacity;
//     }

//     // pub fn voxelColors(self: *const Self, ) !vc.VoxelColor {
//     //     const x = self.list.get(0).?.width;
//     //     const y = self.list.get(0).?.height;
//     //     const z = self.len();
//     //     const colors = vc.newVoxelColor(x, y, z);
//     //     for (0..z) |k| {
//     //         for (0..y) |j| {
//     //             for (0..x) |i| {
//     //                 const img = self.list.get(k).?.image;
//     //                 colors.setVoxelColor(colors.idx(i, j, z), vc.RGBA.R, img.getColor(@intCast(i), @intCast(j)).r);
//     //                 colors.setVoxelColor(colors.idx(i, j, z), vc.RGBA.R, img.getColor(@intCast(i), @intCast(j)).g);
//     //                 colors.setVoxelColor(colors.idx(i, j, z), vc.RGBA.R, img.getColor(@intCast(i), @intCast(j)).b);
//     //                 colors.setVoxelColor(colors.idx(i, j, z), vc.RGBA.A, img.getColor(@intCast(i), @intCast(j)).a);
//     //             }
//     //         }
//     //     }
//     //     return colors;
//     // }
// };

// pub const Image = struct {
//     image: rl.Image,

//     const Self = @This();

//     pub fn load(filename: [:0]const u8) !ImageList {
//         var list = try ImageList.init(1);
//         try list.push(Image{ .image = try rl.loadImage(filename) });
//         return list;
//     }
//     pub fn loadTileSet(filename: [:0]const u8, horizontal: u8, vertical: u8) !ImageList {
//         const image = try Image.load(filename);
//         return (try image.get(0)).?.tileSet(horizontal, vertical);
//     }
//     pub fn deinit(self: Self) void {
//         self.image.unload();
//     }

//     pub fn tileSet(self: *const Self, horizontal: u8, vertical: u8) !ImageList {
//         const capacity = @as(usize, horizontal) * @as(usize, vertical);
//         var list = try ImageList.init(capacity);

//         const hi = @divFloor(self.height(), vertical);
//         const wi = @divFloor(self.width(), horizontal);

//         for (0..horizontal) |h| {
//             for (0..vertical) |v| {
//                 const img = self.image.copyRec(rl.Rectangle{
//                     .height = floatCast(hi),
//                     .width = floatCast(wi),
//                     .x = floatCast(h) * floatCast(wi),
//                     .y = floatCast(v) * floatCast(hi),
//                 });
//                 try list.push(Image{ .image = img });
//             }
//         }

//         return list;
//     }
//     inline fn floatCast(x: anytype) f32 {
//         return @floatFromInt(x);
//     }

//     pub fn height(self: *const Self) i32 {
//         return self.image.height;
//     }
//     pub fn width(self: *const Self) i32 {
//         return self.image.width;
//     }

//     pub fn color(self: *const Self, x: usize, y: usize) rl.Color {
//         return self.image.getColor(@intCast(x), @intCast(y));
//     }
// };
