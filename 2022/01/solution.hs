module Day.Day01
    (main,
    )
where

import AoC (rawExampleInput, rawInput)
import Data.List (sort)
import Data.Text qualified as T

partA :: [[Int]] -> Int
PartA xs = maximum $ map sum xs

main :: IO ()
main = do
    input <- map (map read . words . T.unpack) . T.splitOn "\n\n" <$> rawInput 1
    print (partA input)