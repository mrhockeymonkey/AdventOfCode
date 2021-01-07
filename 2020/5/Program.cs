using System;
using System.IO;
using System.Linq;

namespace _5
{
    class Program
    {
        record BoardingPass(int SeatRow, int SeatColumn)
        {
            public int SeatID => SeatRow * 8 + SeatColumn;
        }

        private static int Search(int[] rows, string directions) {
            var left = rows[0];
            var right = rows[rows.Length -1];

            foreach (var direction in directions)
            {
                int move = (right - left) / 2;
                switch(direction)
                {
                    case 'F':
                    case 'L':
                        // move the right marker
                        right = right - (move + 1);
                        break;
                    case 'B':
                    case 'R':
                        // move the left marker
                        left = left + move + 1;
                        break;
                    default:
                        throw new ArgumentException("Directions can only be LRFB!");
                }
            }

            if (left != right)
            {
                throw new ArgumentException("Invalid directions given!");
            }
            return left;
        }

        private static BoardingPass GetBoardingPass(string directions)
        {
            var rowDirections = directions[0..7];
            var colDirections = directions[7..10];
            var row = Search(Enumerable.Range(0, 128).ToArray(), rowDirections);
            var col = Search(Enumerable.Range(0, 8).ToArray(), colDirections);

            return new BoardingPass(row, col);
        }

        static void Main(string[] args)
        {
            foreach (var directions in new string[] { "FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"})
            {
                var bp = GetBoardingPass(directions);
                Console.WriteLine(bp);
            }

            var boardingPasses = File.ReadAllLines("input.txt")
                .Select(d => GetBoardingPass(d))
                .OrderByDescending(bp => bp.SeatID)
                .ToArray();

            var answer1 = boardingPasses.First();
            Console.WriteLine($"Answer 1: {answer1}");

            for (int i = 0; i < boardingPasses.Length; i++)
            {
                var thisBP = boardingPasses[i];
                var nextBP = boardingPasses[i + 1];

                var expectedSeatID = thisBP.SeatID - 1;
                if (nextBP.SeatID != expectedSeatID) 
                {
                    Console.WriteLine($"Answer 2: {expectedSeatID}");
                    break;
                }
            };
        }
    }
}
