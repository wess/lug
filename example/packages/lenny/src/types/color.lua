--
-- color.lua
-- Culprit
-- 
-- Author: Wess Cope (me@wess.io)
-- Created: 01/13/2022
-- 
-- Copywrite (c) 2022 Wess.io
--

local color = {}

function color:create(red, green, blue)
  return {
    red = red,
    green = green,
    blue = blue
  }
end

function color:hex(hex, alpha)
  return {
    tonumber(string.sub(hex, 2, 3), 16)/256, 
    tonumber(string.sub(hex, 4, 5), 16)/256, 
    tonumber(string.sub(hex, 6, 7), 16)/256, alpha or 1
  }
end

return color