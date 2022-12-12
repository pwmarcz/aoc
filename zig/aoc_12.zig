const std = @import("std");

const Point = struct {
    x: usize,
    y: usize,

    pub fn distance(p1: Point, p2: Point) usize {
        const dx = if (p1.x > p2.x) p1.x - p2.x else p2.x - p1.x;
        const dy = if (p1.y > p2.y) p1.y - p2.y else p2.y - p1.y;
        return dx + dy;
    }
};

const PQ = std.PriorityQueue(Point, *const Grid, Grid.pointCompare);

const PATH_MAX = std.math.maxInt(usize);

const Grid = struct {
    nums: []u8,
    path: []usize,
    width: usize,
    height: usize,
    start: Point,
    end: Point,

    pub fn pointCompare(self: *const Grid, p1: Point, p2: Point) std.math.Order {
        const d1 = self.path[p1.y * self.width + p1.x] + p1.distance(self.end);
        const d2 = self.path[p2.y * self.width + p2.x] + p2.distance(self.end);
        return std.math.order(d1, d2);
    }

    pub fn free(self: *const Grid, allocator: std.mem.Allocator) void {
        allocator.free(self.nums);
        allocator.free(self.path);
    }

    pub fn findPath(self: *const Grid, allocator: std.mem.Allocator) !usize {
        var queue = PQ.init(allocator, self);
        defer queue.deinit();
        try queue.add(self.start);
        self.path[self.start.y * self.width + self.start.x] = 0;

        while (queue.len > 0) {
            const p = queue.remove();
            const pathLen = self.path[p.y * self.width + p.x];
            if (p.x == self.end.x and p.y == self.end.y) {
                return pathLen;
            }
            try self.maybeAdd(&queue, p, -1, 0);
            try self.maybeAdd(&queue, p, 1, 0);
            try self.maybeAdd(&queue, p, 0, -1);
            try self.maybeAdd(&queue, p, 0, 1);
        }
        return error.noPath;
    }

    pub fn printPaths(self: *const Grid) void {
        var y: usize = 0;
        while (y < self.height) : (y += 1) {
            var x: usize = 0;
            while (x < self.width) : (x += 1) {
                std.debug.print("{d} ", .{self.path[y * self.width + x]});
            }
            std.debug.print("\n", .{});
        }
    }

    pub fn maybeAdd(self: *const Grid, queue: *PQ, p: Point, dx: isize, dy: isize) !void {
        if ((dx < 0 and p.x < -dx) or (dx > 0 and @intCast(isize, p.x) + dx >= self.width) or
            (dy < 0 and p.y < -dy) or (dy > 0 and @intCast(isize, p.y) + dy >= self.height)) {
            return;
        }
        const x = @intCast(usize, @intCast(isize, p.x) + dx);
        const y = @intCast(usize, @intCast(isize, p.y) + dy);

        const pNum = self.nums[p.y * self.width + p.x];
        const num = self.nums[y * self.width + x];
        if (num > pNum + 1) {
            return;
        }

        const pathLen = self.path[p.y * self.width + p.x] + 1;
        if (self.path[y * self.width + x] <= pathLen) {
            return;
        }
        self.path[y * self.width + x] = pathLen;
        try queue.add(Point{.x = x, .y = y});
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
        const path = try allocator.alloc(usize, width * height);
        errdefer allocator.free(path);

        var start = Point{.x = 0, .y = 0};
        var end = Point{.x = 0, .y = 0};

        var y: usize = 0;
        while (y < height) : (y += 1) {
            var x: usize = 0;
            while (x < width) : (x += 1) {
                var c = buf[y * (width + 1) + x];
                if (c == 'S') {
                    c = 'a';
                    start.x = x;
                    start.y = y;
                } else if (c == 'E') {
                    c = 'z';
                    end.x = x;
                    end.y = y;
                }

                nums[y * width + x] = c;
                path[y * width + x] = PATH_MAX;
            }
        }

        return Grid{
            .nums = nums,
            .path = path,
            .width = width,
            .height = height,
            .start = start,
            .end = end,
        };
    }
};

const GridError = error{
    badDimensions,
    unexpectedChar,
    noPath,
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const stdin = std.io.getStdIn();
    const buf: []u8 = try stdin.readToEndAlloc(allocator, std.math.maxInt(usize));
    defer allocator.free(buf);

    const grid = try Grid.create(buf, allocator);
    defer grid.free(allocator);

    std.debug.print("{d}\n", .{try grid.findPath(allocator)});
    //grid.printPaths();
}
