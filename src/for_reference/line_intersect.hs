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

-- Returns the parameters `(t, s)` at which `start` and `end` intersect.
-- `t` is the parameter for `start`
-- `s` is the parameter for `end`
intersectTwoLines :: (GridPoint, GridPoint) -> (GridPoint, GridPoint) -> (Float, Float)
intersectTwoLines
  start@(GridPoint {x = (x0, fx0), y = (y0, fy0)}, GridPoint {x = (x1, fx1), y = (y1, fy1)})
  end@(GridPoint {x = (x2, fx2), y = (y2, fy2)}, GridPoint {x = (x3, fx3), y = (y3, fy3)}) =
    let (ax, ay) = gridPointToFloat . fst $ start
        (bx, by) = gridPointToFloat . snd $ start
        (cx, cy) = gridPointToFloat . fst $ end
        (dx, dy) = gridPointToFloat . snd $ end

        bax = bx - ax
        bay = by - ay
        dcx = dx - cx
        dcy = dy - cy
        cax = cx - ax
        cay = cy - ay
        gradientA = bay / bax
        gradientB = dcy / dcx

        -- We can afford to set s to an arbitrary value since we don't care at all what the value is
        -- unless it's in the interval [0, 1]
        s = if gradientA == gradientB then -1 else (bax * cay - bay * cax) / (bay * dcx - bax * dcy)

        t =
          if x0 == x1
            then (s * dcy + cy - ay) / bay
            else (s * dcx + cx - ax) / bax
     in (t, s)

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

      forEachHorizontal :: (GridPoint, GridPoint) -> [GridPoint]
      forEachHorizontal (c, d) =
        let (t, s) = intersectTwoLines (start, end) (c, d)
         in [floatToGridPoint (ax + (bx - ax) * t, ay + (by - ay) * t) | s >= 0, s <= 1, t >= 0, t <= 1]

      pointsOnHorizontals = map forEachHorizontal horizontalLines
   in concat pointsOnHorizontals
  where
    verticalMap :: Int -> (GridPoint, GridPoint)
    verticalMap i = (GridPoint {x = (i, 0), y = (0, 0)}, GridPoint {x = (i, 0), y = (height, 0)})

    horizontalMap :: Int -> (GridPoint, GridPoint)
    horizontalMap i = (GridPoint {x = (0, 0), y = (i, 0)}, GridPoint {x = (width, 0), y = (i, 0)})

tests =
  [ (GridPoint {x = (25, 1), y = (20, 0)}, GridPoint {x = (23, 0), y = (20, 0)}),
    (GridPoint {x = (25, 1), y = (20, 0)}, GridPoint {x = (25, 1), y = (25, 0)}),
    (GridPoint {x = (25, 1), y = (20, 3)}, GridPoint {x = (20, 3), y = (25, 1)}),
    (GridPoint {x = (20, 3), y = (25, 1)}, GridPoint {x = (25, 1), y = (20, 3)})
  ]

main :: IO ()
main = mapM_ (putStrLn . printTest) tests
  where
    printTest (a, b) = "A=" ++ show a ++ ", B=" ++ show b ++ ":\n" ++ unlines (map show (intersectWithGrid a b))