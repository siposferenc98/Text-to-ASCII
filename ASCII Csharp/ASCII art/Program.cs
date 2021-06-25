using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ASCII_art
{
    class Program
    {
        static void Main(string[] args)
        {
            List<string> betuk = new List<string>();
            StreamReader sr = new StreamReader("alphabet.txt");
            int L = 4;
            int H = 5;
            Console.WriteLine("Írd be a szövegedet");
            string T = Console.ReadLine();

            for (int i = 0; i < H; i++)
            {
                string ROW = sr.ReadLine();
                for (int j = 0; j < ROW.Length; j += L)
                {
                    string sorsubstring = ROW.Substring(j, L);
                    betuk.Add(sorsubstring);
                }

            }
            sr.Close();
            string[,] betukmatrix = new string[H, 28];
            int a = 0;
            int b = 0;
            int c = 0;
            while (a < H)
            {
                while (b < 28)
                {
                    betukmatrix[a, b] = betuk[c];
                    b++;
                    c++;
                }
                a++;
                b = 0;
            }

            //Test
            /*for (int i = 0; i < 5; i++)
            {
                for (int j = 0; j < 27; j++)
                {
                    Console.Error.Write(betukmatrix[i,j]);
                }
                Console.Error.WriteLine();


            }

            for (int i = 0; i < 5; i++)
            {

                    Console.Error.Write(betukmatrix[i,0]);

                Console.Error.WriteLine();

            }

                for (int i = 0; i < 5; i++)
            {

                    Console.Write(betukmatrix[i,4]);

                Console.WriteLine();


            }*/




            char[] abc = { 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',' ','?' };
            char[] bemenet = T.ToUpper().ToCharArray();
            List<int> indexek = new List<int>();
            int nincs = 0;
            for (int j = 0; j < bemenet.Length; j++)
            {
                for (int i = 0; i < abc.Length; i++)
                {
                    nincs++;
                    if (abc[i] == bemenet[j])
                    {
                        indexek.Add(i);
                        nincs = 0;
                    }
                    else if (nincs == abc.Length)
                    {
                        indexek.Add(27);
                    }

                }
                nincs = 0;
            }

            string[,] answer = new string[H, indexek.Count];
            for (int i = 0; i < H; i++)
            {
                for (int j = 0; j < indexek.Count; j++)
                {
                    answer[i, j] = betukmatrix[i, indexek[j]];
                }
            }

            for (int i = 0; i < H; i++)
            {
                for (int j = 0; j < indexek.Count; j++)
                {
                    Console.Write(answer[i, j]);
                }
                Console.WriteLine();
            }

            Console.ReadKey();
        }
    }
}
