--
-- vector.lua
-- lenny
-- 
-- Author: wess (me@wess.io)
-- Created: 01/26/2022
-- 
-- Copywrite (c) 2022 Wess.io
--


local vector = {}

function vector.new(x, y)
  return {
    x = x or 0,
    y = y or 0
  }
end

function vector.add(lhs, rhs)
  return {
    x = lhs.x + rhs.x,
    y = lhs.y + rhs.y
  }
end

function vector.subtract(lhs, rhs)
  return {
    x = lhs.x - rhs.x,
    y = lhs.y - rhs.y
  }
end

function vector.multiply(lhs, rhs)
  return {
    x = lhs.x * rhs.x,
    y = lhs.y * rhs.y
  }
end

function vector.magnitude(v)
  return math.sqrt(
    v.x * v.x + v.y * v.y
  )
end

function vector.normalize(v)
  local mag = vector.magnitude(v)

  return {
    x = v.x / mag,
    y = v.y / mag
  }
end

return vector