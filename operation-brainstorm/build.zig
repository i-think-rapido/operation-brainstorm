const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});




    // main program module
    const exe_mod = b.addExecutable(.{
        .name = "operation-brainstorm",
        .root_source_file = b.path("./src/main.zig"),

        .target = target,
        .optimize = optimize,
    });
    exe_mod.linkLibC();

    // add include paths
    exe_mod.addIncludePath(b.path("../voxel-color/target"));
    exe_mod.addObjectFile(b.path("../voxel-color/target/release/libvoxel_color.a"));
    // exe_mod.installHeader(b.path("../voxel-color/target/voxel-color.h"), "voxel-color/voxel-color.h");

    // raylib dependency
    const raylib_dep = b.dependency("raylib_zig", .{
        .target = b.graph.host,
        .optimize = optimize,
    });
    const raylib_mod = raylib_dep.module("raylib"); // main raylib module
    const raygui_mod = raylib_dep.module("raygui"); // raygui module
    
    exe_mod.root_module.addImport("raylib", raylib_mod);
    exe_mod.root_module.addImport("models", raylib_mod);
    exe_mod.root_module.addImport("raygui", raygui_mod);

    // artifacts
//    b.installArtifact(raylib_dep.artifact("raylib_zig")); // install raylib artifact

    // executable
    const run_exe = b.addRunArtifact(exe_mod);

    // build steps
    const run_step = b.step("run", "operation brainstorm exe");
    run_step.dependOn(&run_exe.step);
}
