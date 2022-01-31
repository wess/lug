--
-- rect.lua
-- Culprit
-- 
-- Author: Wess Cope (me@wess.io)
-- Created: 01/13/2022
-- 
-- Copywrite (c) 2022 Wess.io
--

local rect = {}
function rect:create(x, y, width, height)
  return {
    x = x,
    y = y,
    width = width,
    height = height
  }
end

return rect