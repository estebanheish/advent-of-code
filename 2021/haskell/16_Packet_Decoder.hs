--- Day 16: Packet Decoder ---

import Data.Char (digitToInt)
import Data.List (foldl')
import Text.Parsec.String (Parser)
import Text.Parsec

main :: IO ()
main = do
  input <- readFile "input/16_Packet_Decoder.input"
  let input' = (concat . (hexToBinary <$>) . reverse . drop 1 . reverse) input
  case parse parsePacket "parse" input' of 
    Right r -> print (countVersions [r]) >> print (evaluate r)  -- Part 1: 866 -- Part 2: 1392637195518
    Left e    -> print e

type Version = Int
type TypeID  = Int
type Literal = Int

data Packet = Op      Version TypeID [Packet]
            | Literal Version Literal
            deriving (Show)

countVersions = foldl (\acc x -> version x + acc) 0
    where version j = case j of
                        Op v _ p    -> v + countVersions p
                        Literal v _ -> v

evaluate :: Packet -> Int
evaluate (Op _ t ps)   = ope t (evaluate <$> ps)
evaluate (Literal _ l) = l

ope o = 
  case o of
    0 -> sum
    1 -> product 
    2 -> minimum 
    3 -> maximum 
    5 -> (\x -> if head x > last x then 1 else 0) -- greater than
    6 -> (\x -> if head x < last x then 1 else 0) -- less than
    7 -> (\x -> if head x == last x then 1 else 0) -- equal

parsePacket :: Parser Packet
parsePacket = do 
    version <- toDec <$> count 3 (oneOf "10")
    typeId <- toDec <$> count 3 (oneOf "10")
    if typeId  == 4 then 
        do p <- many (char '1' >> count 4 (oneOf "10"))
           zero <- char '0'
           u <- count 4 (oneOf "10")
           let literal = toDec (concat p ++ u)
           return (Literal version literal)
    else 
        do lengthId <- oneOf "10"
           if lengthId == '1' then
               do nPackets <- toDec <$> count 11 (oneOf "10")
                  packets <- count nPackets parsePacket
                  return (Op version typeId packets)
            else 
                do length <- toDec <$> count 15 (oneOf "10")
                   packets <- count length (oneOf "10")
                   case parse (many parsePacket) "op" packets of
                     Right r -> return (Op version typeId r)
                     Left l -> parserFail ""
                 
toDec :: String -> Int
toDec = foldl' (\acc x -> acc * 2 + digitToInt x) 0

hexToBinary :: Char -> String
hexToBinary c = case c of
    '0' -> "0000"
    '1' -> "0001"
    '2' -> "0010"
    '3' -> "0011"
    '4' -> "0100"
    '5' -> "0101"
    '6' -> "0110"
    '7' -> "0111"
    '8' -> "1000"
    '9' -> "1001"
    'A' -> "1010"
    'B' -> "1011"
    'C' -> "1100"
    'D' -> "1101"
    'E' -> "1110"
    'F' -> "1111"
