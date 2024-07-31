const std = @import("std");
const testing = std.testing;

pub fn dayOne() u32 {
    const allocator = std.heap.page_allocator;
    const buf = std.fs.cwd().readFileAlloc(allocator, "example_one.txt", std.math.maxInt(usize)) catch |err| {
        std.debug.print("{}\n", .{err});
        return 0;
    };
    var it = std.mem.tokenize(u8, buf, "\n");
    while (it.next()) |line| {
        var line_nums: [2]i32 = [_]i32{ -1, -1 };
        var sum: i32 = 0;
        for (line) |c| {
            if ((c > 47) and (c < 58)) {
                std.debug.print("{c}", .{c});
                const val = (c - '0');
                push(&line_nums, val);
            }
            sum += line_nums[0] + line_nums[1];
        }
        std.debug.print("\n", .{});
        std.debug.print("Line total is {d}\n", .{sum});
    }

    allocator.free(buf);
    return 9;
}

pub fn pop(arr: *[]i32) void {
    if (arr[1] != -1) {
        arr[1] = -1;
        return;
    }
    if (arr[0] != -1) {
        arr[0] = -1;
        return;
    }
}
pub fn push(arr: *[]i32, val: i32) void {
    if (arr[0] != -1) {
        arr[0] = val;
        return;
    }
    if (arr[1] != -1) {
        arr[1] = val;
        return;
    }
}

test push {}
