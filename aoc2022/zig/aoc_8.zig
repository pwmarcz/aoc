const std = @import("std");

const Grid = struct {
    nums: []u8,
    seen: []u8,
    scores: []usize,
    width: usize,
    height: usize,

    pub fn free(self: *const Grid, allocator: std.mem.Allocator) void {
        allocator.free(self.nums);
        allocator.free(self.seen);
    }

    fn checkSeen(self: *const Grid, lvl: *i8, x: usize, y: usize) void {
        const idx = y * self.width + x;
        if (self.nums[idx] <= lvl.*) {
            self.seen[idx] -= 1;
        } else {
            lvl.* = @intCast(i8, self.nums[idx]);
        }
    }

    fn checkScore(
        self: *const Grid,
        view: *[10]usize,
        x: usize,
        y: usize,
    ) void {
        const idx = y * self.width + x;
        const num = self.nums[idx];
        const score = view.*[num];
        self.scores[idx] *= score;

        var i: usize = 0;
        while (i < 10) : (i += 1) {
            if (i <= num) {
                view.*[i] = 1;
            } else {
                view.*[i] += 1;
            }
        }
    }

    fn sweepWith(
        self: *const Grid,
        comptime T: type,
        comptime f: fn(*const Grid, *T, usize, usize) void,
        init: T,
    ) void {
        {var y: usize = 0;
        while (y < self.height) : (y += 1) {
            var state = init;
            var x: usize = 0;
            while (x < self.width) : (x += 1) {
                f(self, &state, x, y);
            }

            state = init;
            x = self.width;
            while (x > 0) : (x -= 1) {
                f(self, &state, x - 1, y);
            }
        }}

        {var x: usize = 0;
        while (x < self.width) : (x += 1) {
            var state = init;
            var y: usize = 0;
            while (y < self.height) : (y += 1) {
                f(self, &state, x, y);
            }

            state = init;
            y = self.height;
            while (y > 0) : (y -= 1) {
                f(self, &state, x, y - 1);
            }
        }}
    }

    pub fn sweep(self: *const Grid) void {
        self.sweepWith(i8, Grid.checkSeen, -1);
        self.sweepWith([10]usize, Grid.checkScore, [_]usize{0} ** 10);
    }

    pub fn countSeen(self: *const Grid) usize {
        var count: usize = 0;
        for (self.seen) |nSeen| {
            if (nSeen > 0) count += 1;
        }
        return count;
    }

    pub fn maxScore(self: *const Grid) usize {
        var best: usize = 0;
        for (self.scores) |score| {
            if (score > best) best = score;
        }
        return best;
    }

    pub fn create(buf: []u8, allocator: std.mem.Allocator) !Grid {
        if (buf.len == 0) return error.badDimensions;
        const size = if (buf[buf.len - 1] == '\n') buf.len else buf.len + 1;

        var newline: ?usize = null;
        for (buf) |c, i| {
            if (c == '\n') {
                newline = i;
                break;
            }
        }

        const width = newline orelse size - 1;
        if (width == 0) return error.badDimensions;
        if (size % (width + 1) != 0) return error.badDimensions;
        const height = size / (width + 1);

        const nums = try allocator.alloc(u8, width * height);
        errdefer allocator.free(nums);
        const seen = try allocator.alloc(u8, width * height);
        errdefer allocator.free(seen);
        const scores = try allocator.alloc(usize, width * height);
        errdefer allocator.free(scores);


        var y: usize = 0;
        while (y < height) : (y += 1) {
            var x: usize = 0;
            while (x < width) : (x += 1) {
                const c = buf[y * (width + 1) + x];
                if (!('0' <= c and c <= '9')) {
                    std.debug.print("unexpected: {c}\n", .{c});
                    return GridError.unexpectedChar;
                }
                nums[y * width + x] = c - '0';
                seen[y * width + x] = 4;
                scores[y * width + x] = 1;
            }
        }

        return Grid{
            .nums = nums,
            .seen = seen,
            .scores = scores,
            .width = width,
            .height = height,
        };
    }
};

const GridError = error{
    badDimensions,
    unexpectedChar,
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const stdin = std.io.getStdIn();
    const buf: []u8 = try stdin.readToEndAlloc(allocator, std.math.maxInt(usize));
    defer allocator.free(buf);

    const grid = try Grid.create(buf, allocator);
    defer grid.free(allocator);

    grid.sweep();

    std.debug.print("{d} {d}\n", .{grid.countSeen(), grid.maxScore()});
}
