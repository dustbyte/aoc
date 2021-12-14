using System;

public class Number {
    public int field { get ; set; } = 0;
    public int len { get ; set; } = 0;
}

class Day8
{
    public static void Main(string[] args)
    {
        if (args.Length < 1)
        {
            Console.Error.WriteLine("usage: d8 file");
            Environment.Exit(-1);
        }

        int sum1478 = 0;
        int sum = 0;

        foreach (string line in System.IO.File.ReadLines(args[0]))
        {
            var sections = line.Split('|');
            var numbase = sections[0].Trim(' ').Split(' ');
            var output = sections[1].Trim(' ').Split(' ');

            Number[] input = new Number[10];
            int[] numbers = new int[10];
            int number = 0;

            for (int i = 0; i < 10; i++) {
                input[i] = new Number();
                input[i].len = numbase[i].Length;
                input[i].field = StrToField(numbase[i]);

                switch (input[i].len) {
                    case 2:
                        numbers[1] = input[i].field;
                        break;
                    case 4:
                        numbers[4] = input[i].field;
                        break;
                    case 3:
                        numbers[7] = input[i].field;
                        break;
                    case 7:
                        numbers[8] = input[i].field;
                        break;
                }
            }

            numbers[3] = GetNumberValue(input, 5, numbers[7]);
            numbers[9] = GetNumberValue(input, 6, numbers[3]);
            numbers[6] = GetNumberValue(input, 6, ~numbers[1]);
            numbers[2] = GetNumberValue(input, 5, ~numbers[4]);
            numbers[5] = GetNumberValue(input, 5, ~numbers[2]);
            numbers[0] = GetNumberValue(input, 6, ~numbers[5]);

            foreach (string digit in output) {
                int field = StrToField(digit);

                for (int i = 0; i < 10; i++) {
                    if (numbers[i] == field) {
                        if (i == 1 || i == 4 || i == 7 || i == 8) {
                            sum1478++;
                        }

                        number = number * 10 + i;
                        break;
                    }
                }
            }

            sum += number;
        }

        Console.WriteLine(sum1478);
        Console.WriteLine(sum);
    }

    public static int StrToField(string str) {
        int result = 0;

        foreach (char c in str) {
            result |= 1 << (c - 'a');
        }

        return result;
    }

    public static int GetNumberValue(Number[] input, int len, int cmp) {
        foreach (Number num in input) {
            if (num.len == len && ((num.field & 0x7F) & (cmp & 0x7F)) == (cmp & 0x7F)) {
                return num.field;
            }
        }
        return 0;
    }
}
