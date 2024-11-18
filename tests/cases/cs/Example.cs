using System;

namespace TestCases
{
    class Program
    {
        static void Main(string[] args)
        {
            int[] numbers = { 1, 2, 3, 4, 5 };
            int i = 0;

            // While loop
            while (i < numbers.Length)
            {
                // If statement
                if (numbers[i] % 2 == 0)
                {
                    Console.WriteLine($"{numbers[i]} is even.");
                }
                else
                {
                    Console.WriteLine($"{numbers[i]} is odd.");
                }
                i++;
            }

            // For loop
            for (int j = 0; j < numbers.Length; j++)
            {
                Console.WriteLine($"Number at index {j} is {numbers[j]}.");
            }
        }
    }
}
