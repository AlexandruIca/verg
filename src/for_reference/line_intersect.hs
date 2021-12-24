import GHC.Base (VecElem (Int16ElemRep))

pixelSize :: Int
pixelSize = 4

width :: Int
width = 600

height :: Int
height = 600

data GridPoint = GridPoint {x :: (Int, Int), y :: (Int, Int)} deriving (Show)

gridPointToFloat :: GridPoint -> (Float, Float)
gridPointToFloat point =
  let (ix, ifx) = x point
      (iy, ify) = y point
      (x', fx') = (fromIntegral ix :: Float, fromIntegral ifx :: Float)
      (y', fy') = (fromIntegral iy :: Float, fromIntegral ify :: Float)
      pixelSize' = fromIntegral pixelSize :: Float
   in ( x' + fx' / pixelSize',
        y' + fy' / pixelSize'
      )

floatToGridPoint :: (Float, Float) -> GridPoint
floatToGridPoint (x, y) = GridPoint {x = roundOnAxis x, y = roundOnAxis y}
  where
    roundOnAxis :: Float -> (Int, Int)
    roundOnAxis axis =
      let pixelLength = 1 / (fromIntegral pixelSize :: Float)
          f = (snd . properFraction) axis
          cellIndex = f / pixelLength
       in ((fst . properFraction) axis, floor cellIndex)

-- Generates numbers in range [start, end) with step (similar to python)
range :: Int -> Int -> Int -> [Int]
range start end step = go start end step []
  where
    go :: Int -> Int -> Int -> [Int] -> [Int]
    go _ _ 0 _ = []
    go start end step acc
      | start == end = acc
      | otherwise = go (start + step) end step (acc ++ [start])

intersectWithGrid :: GridPoint -> GridPoint -> [GridPoint]
intersectWithGrid start end =
  let ((x0, fx0), (y0, fy0)) = (x start, y start)
      ((x1, fx1), (y1, fy1)) = (x end, y end)

      dirX = signum (x1 - x0)
      dirY = signum (y1 - y0)

      (startX, endX) = if dirX < 0 then (x0 + 1, x1 - 1) else (x0 - 1, x1 + 1)
      (startY, endY) = if dirY < 0 then (y0 + 1, y1 - 1) else (y0 - 1, y1 + 1)

      (ax, ay) = gridPointToFloat start
      (bx, by) = gridPointToFloat end

      verticalRange = range startX endX dirX
      horizontalRange = range startY endY dirY

      verticalLines = map verticalMap verticalRange
      horizontalLines = map horizontalMap horizontalRange

      forEachHorizontal :: ((Float, Float), (Float, Float)) -> [GridPoint]
      forEachHorizontal ((cx, cy), (dx, dy)) =
        let dx1 = bx - ax
            dy1 = by - ay
            dx2 = dx - cx
            dy2 = dy - cy
            dx3 = cx - ax
            dy3 = cy - ay
            gradientA = dy1 / dx1
            gradientB = dy2 / dx2

            s = if gradientA == gradientB then -1 else (dx1 * dy3 - dy1 * dx3) / (dy1 * dx2 - dx1 * dy2)

            -- TODO: Merge with dx<n> names
            dcx = dx - cx
            bax = bx - ax
            dcy = dy - cy
            bay = by - ay

            t =
              if x0 == x1
                then (s * dcy + cy - ay) / bay
                else (s * dcx + cx - ax) / bax
         in [floatToGridPoint (ax + bax * t, ay + bay * t) | s >= 0, s <= 1, t >= 0, t <= 1]

      pointsOnHorizontals = map forEachHorizontal horizontalLines
   in concat pointsOnHorizontals
  where
    verticalMap :: Int -> ((Float, Float), (Float, Float))
    verticalMap i = ((fromIntegral i, 0), (fromIntegral i, fromIntegral height))

    horizontalMap :: Int -> ((Float, Float), (Float, Float))
    horizontalMap i = ((0, fromIntegral i), (fromIntegral width, fromIntegral i))

main :: IO ()
main = print $ intersectWithGrid GridPoint {x = (25, 1), y = (20, 0)} GridPoint {x = (23, 0), y = (20, 0)}

--main = print $ intersectWithGrid GridPoint {x = (25, 1), y = (20, 0)} GridPoint {x = (25, 1), y = (25, 0)}
--main = print $ intersectWithGrid GridPoint {x = (25, 1), y = (20, 3)} GridPoint {x = (20, 3), y = (25, 1)}
--main = print $ intersectWithGrid GridPoint {x = (20, 3), y = (25, 1)} GridPoint {x = (25, 1), y = (20, 3)}