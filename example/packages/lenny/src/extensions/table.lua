--
-- table.lua
-- lenny
-- 
-- Author: wess (me@wess.io)
-- Created: 01/26/2022
-- 
-- Copywrite (c) 2022 Wess.io
--

function table.filter(tbl, cond)
  local result = {}
  
  for k, v in pairs(tbl) do
    if cond(v, k) then
      result[k] = v
    end
  end

  return result
end

function table.map(tbl, cond)
  local result = {}

  for k, v in pairs(tbl) do
    result[k] = cond(v, k)
  end

  return result
end

function table.reduce(tbl, init, cond)
  local result = init

  for _, v in pairs(tbl) do
    result = cond(result, v)
  end

  return result
end

function table.empty(tbl)
  return #tbl == 0
end

