///
/// Brain MRI 3D Viewer
///
///
///
///
///
pub fn main() !void {
    //try dbg("Operation {s}.\n", .{"Brainstorm"});
    return show();
}

const FRAME_RATE = 24;

// imports
const dbg = @import("./dbg.zig").dbg;
const std = @import("std");
const rl = @import("raylib");
const vc = @import("voxel-color");
const lib = @import("project-brainstorm-lib");
const img = @import("images.zig");

// game types
const ColorPalette = enum {
    Red,
    Blue,
    Green,
};

const voxel_size = 4;

const dimensions = [_]u16{ 4, 3, 1 };

pub fn show() !void {
    // Initialization
    //--------------------------------------------------------------------------------------

    rl.setConfigFlags(.{ .fullscreen_mode = true, .window_maximized = true });
    rl.initWindow(0, 0, "raylib-zig [core] example - basic window");
    defer rl.closeWindow(); // Close window and OpenGL context
    rl.maximizeWindow();
    //    rl.setWindowFocused();
    //--------------------------------------------------------------------------------------

    // Define the camera to look into our 3d world
    var camera = rl.Camera{
        .position = .init(10, 10, 10),
        .target = .init(0, 0, 0),
        .up = .init(0, 1, 0),
        .fovy = 45,
        .projection = .perspective,
    };

    rl.disableCursor(); // Limit cursor to relative movement inside the window
    rl.setTargetFPS(FRAME_RATE); // Set our game to run at 60 frames-per-second

    // config
    var colors = try img.VoxelColors.fromFile("./data/brain.png");

    var container = try create_model(0.1);
    try initialize_shader_dim(&container.shader, &colors);
    try initialize_shader_colors(&container.shader, &colors);
    try initialize_shader_positions(&container.shader, &colors);

    while (!rl.windowShouldClose()) {
        rl.updateCamera(@constCast(&camera), rl.CameraMode.first_person);

        rl.beginDrawing();
        rl.clearBackground(rl.Color.ray_white);

        rl.beginMode3D(camera);
        rl.drawModel(container.model, rl.Vector3{ .x = 0.0, .y = 0.0, .z = 0.0 }, 1.0, colors.get(0));
        rl.endMode3D();

        rl.drawText("Instanced Cubes", 10, 10, 20, rl.Color.light_gray);
        rl.endDrawing();
    }

    rl.unloadShader(container.shader);
    rl.unloadModel(container.model);
    rl.closeWindow();

    _ = try dbg("exiting...", .{});
}

const Container = extern struct {
    shader: rl.Shader,
    model: rl.Model,
};
fn create_model(size: f32) !Container {
     // Load model and shader
    const model = try rl.loadModelFromMesh(rl.genMeshCube(size, size, size));
    const shader = try rl.loadShader("instanced_cube.vs", "instanced_cube.fs");

    // Dimensions
    // Set shader locations
    model.materials[0].shader = shader;

    return .{ .shader = shader, .model = model };
}
fn initialize_shader_dim(shader: *rl.Shader, colors: *img.VoxelColors) !void {
    const loc_dimX = rl.getShaderLocation(shader.*, "dimX");
    const loc_dimY = rl.getShaderLocation(shader.*, "dimY");
    const loc_dimZ = rl.getShaderLocation(shader.*, "dimZ");

    // Set shader locations
    rl.setShaderValue(shader.*, loc_dimX, &colors.dimX, rl.ShaderUniformDataType.int);
    rl.setShaderValue(shader.*, loc_dimY, &colors.dimY, rl.ShaderUniformDataType.int);
    rl.setShaderValue(shader.*, loc_dimZ, &colors.dimZ, rl.ShaderUniformDataType.int);
}
fn initialize_shader_colors(shader: *rl.Shader, colors: *img.VoxelColors) !void {
    const loc_colors = rl.getShaderLocation(shader.*, "colors");

    const arr = try colors.array();
    defer img.VoxelColors.dearray(arr);

    rl.setShaderValueV(shader.*, loc_colors, @ptrCast(&arr), rl.ShaderUniformDataType.int, @intCast(colors.size() * 4));
}
fn initialize_shader_positions(shader: *rl.Shader, colors: *img.VoxelColors) !void {
    const loc_positions = rl.getShaderLocation(shader.*, "positions");

    const arr = try create_positions(colors);
    defer std.heap.page_allocator.free(arr);

    rl.setShaderValueV(shader.*, loc_positions, @ptrCast(&arr), rl.ShaderUniformDataType.vec3, @intCast(colors.size()));
}

fn create_positions(colors: *img.VoxelColors) ![]rl.Vector3 {
    const allocator = std.heap.page_allocator;
    var positions = try allocator.alloc(rl.Vector3, colors.size());

    for (0..dimensions[0]) |x| {
        for (0..dimensions[1]) |y| {
            for (0..dimensions[2]) |z| {
                positions[try colors.index(x, y, z)] = rl.Vector3{ .x = @floatFromInt(x * voxel_size), .y = @floatFromInt(y * voxel_size), .z = @floatFromInt(z * voxel_size) };
            }
        }
    }

    return positions;
}

