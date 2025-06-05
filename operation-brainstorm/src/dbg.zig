
const std = @import("std");

pub fn dbg(comptime format: []const u8, args: anytype) !@TypeOf(args) {
    var buffer: [2][4096]u8 = undefined;
    const rendered = try std.fmt.bufPrint(&buffer[0], format, args);
    const text = try std.fmt.bufPrint(&buffer[1], "_______________: {s}", .{rendered});
    std.log.debug("{s}", .{text});
    return args;
}
