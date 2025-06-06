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
    const colors = img.VoxelColor.init(3, 2, 1);
    dbg("colors {}", .{ colors });
    // const imageList = try img.Image.loadTileSet("./data/brain.png", 6, 3);
    // defer imageList.deinit();

    // const voxels = try imageList.voxelColors();
    // dbg("{}", .{ voxels });

    // var width_correction: f32 = @floatFromInt(imageList.width());
    // var height_correction: f32 = @floatFromInt(imageList.height());
    // width_correction /= 2.0;
    // height_correction /= 2.0;

    const container = try create_model();

    while (!rl.windowShouldClose()) {
        rl.updateCamera(@constCast(&camera), rl.CameraMode.first_person);

        rl.beginDrawing();
        rl.clearBackground(rl.Color.ray_white);

        rl.beginMode3D(camera);
        rl.drawModel(container.model, rl.Vector3{ .x = 0.0, .y = 0.0, .z = 0.0 }, 1.0, rl.Color.pink);
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
fn create_model() !Container {
     // Load model and shader
    const model = try rl.loadModelFromMesh(rl.genMeshCube(1.0, 1.0, 1.0));
    const shader = try rl.loadShader("instanced_cube.vs", "instanced_cube.fs");

    // Dimensions
    const dimX = 1;
    const dimY = 3;
    const dimZ = 2;
    const spacing = rl.Vector3{ .x = 2, .y = 2, .z = 2 };
    const total = dimX * dimY * dimZ;

    // Set shader locations
    const loc_dimX = rl.getShaderLocation(shader, "dimX");
    const loc_dimY = rl.getShaderLocation(shader, "dimY");
    const loc_dimZ = rl.getShaderLocation(shader, "dimZ");
    const loc_spacing = rl.getShaderLocation(shader, "spacing");
    const loc_colors = rl.getShaderLocation(shader, "cubeColors");

    // Set static uniforms
    rl.setShaderValue(shader, loc_dimX, &dimX, rl.ShaderUniformDataType.int);
    rl.setShaderValue(shader, loc_dimY, &dimY, rl.ShaderUniformDataType.int);
    rl.setShaderValue(shader, loc_dimZ, &dimZ, rl.ShaderUniformDataType.int);
    rl.setShaderValue(shader, loc_spacing, &spacing, rl.ShaderUniformDataType.vec3);

    // Per-instance color array
    var colors: [256]rl.Vector4 = undefined;
    for (0..total) |i| {
        colors[i] = rl.Vector4{
            .x = @as(f32, @floatFromInt(rl.getRandomValue(0, 100))) / 100.0,
            .y = @as(f32, @floatFromInt(rl.getRandomValue(0, 100))) / 100.0,
            .z = @as(f32, @floatFromInt(rl.getRandomValue(0, 100))) / 100.0,
            .w = 1.0,
        };
    }
    rl.setShaderValueV(shader, loc_colors, &colors, rl.ShaderUniformDataType.vec4, total);

    model.materials[0].shader = shader;

    return .{ .shader = shader, .model = model };
}