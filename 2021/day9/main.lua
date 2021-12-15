#!/usr/bin/env lua

function abort(msg)
    io.stderr:write(msg .. "\n")
    os.exit(-1)
end

Stack = {}

function Stack:new()
    local object = {stack = {}, visited = {}}
    setmetatable(object, self)
    self.__index = self
    return object
end

function Stack:push(row, col)
    local key = "{" .. row .. "," .. col .. "}"
    if self.visited[key] == nil then
        self.visited[key] = true
        table.insert(self.stack, {row, col})
    end
end

function Stack:pop()
    return table.remove(self.stack, 1)
end

function Stack:len()
    return #self.stack
end

function basin_size(field, row, col)
    local size = 0
    local stack = Stack:new()
    stack:push(row, col)

    while stack:len() > 0 do
        local point = stack:pop()
        local row = point[1]
        local col = point[2]

        if field[row][col] < 9 then
            size = size + 1

            if row > 1 then stack:push(row - 1, col) end
            if row < #field then stack:push(row + 1, col) end
            if col > 1 then stack:push(row, col - 1) end
            if col < #field[row] then stack:push(row, col + 1) end
        end
    end

    return size
end

if #arg == 0 then
    abort("usage: " .. arg[0] .. " input_file")
end

local field = {}
local fh = io.open(arg[1])
if fh == nil then abort("No such file or directory: " .. arg[1]) end
local lineno = 1
while true do
    local line = fh:read("*line")
    if line == nil then break end
    field[lineno] = {}
    for num=1, #line do
        field[lineno][num] = tonumber(string.sub(line, num, num))
    end
    lineno = lineno + 1
end
fh:close();

local basins = {}
local lowsum = 0
for row=1,#field do
    for col=1,#field[row] do
        if not (col > 1 and field[row][col] >= field[row][col - 1]) and
            not (col < #field[row] and field[row][col] >= field[row][col + 1]) and
            not (row > 1 and field[row][col] >= field[row - 1][col]) and
            not (row < #field and field[row][col] >= field[row + 1][col]) then
            lowsum = lowsum + 1 + field[row][col]
            basins[#basins+1] = basin_size(field, row, col)
        end
    end
end

table.sort(basins)

print(lowsum)
print(basins[#basins] * basins[#basins - 1] * basins[#basins - 2])
