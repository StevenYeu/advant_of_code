const std = @import("std");
const testing = std.testing;

pub fn dayOne() i32 {
    const allocator = std.heap.page_allocator;
    const buf = std.fs.cwd().readFileAlloc(allocator, "example_one.txt", std.math.maxInt(usize)) catch |err| {
        std.debug.print("{}\n", .{err});
        return 0;
    };
    var it = std.mem.tokenize(u8, buf, "\n");
    var sum: i32 = 0;
    while (it.next()) |line| {
        var line_nums: [2]i32 = [_]i32{ -1, -1 };
        var line_sum: i32 = 0;
        for (line) |c| {
            if ((c > 47) and (c < 58)) {
                const val = (c - '0');
                push(&line_nums, val);
            }
            if (line_nums[1] == -1) {
                line_nums[1] = line_nums[0];
            }
        }
        line_sum = (line_nums[0] * 10) + line_nums[1];
        sum += line_sum;
    }

    allocator.free(buf);
    return sum;
}

pub fn pop(arr: *[2]i32) void {
    if (arr[1] != -1) {
        arr[1] = -1;
        return;
    }
    if (arr[0] != -1) {
        arr[0] = -1;
        return;
    }
}
pub fn push(arr: *[2]i32, val: i32) void {
    if (arr[0] == -1) {
        arr[0] = val;
        return;
    }
    if (arr[1] == -1) {
        arr[1] = val;
        return;
    }
    arr[1] = val;
}

test push {}
