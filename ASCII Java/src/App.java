import java.util.*;
import java.io.*;
public class App {
    public static void main(String[] args) throws Exception {
        int L = 4;
        int H = 5;
        ArrayList<String> betuk = new ArrayList<String>();
        Scanner in = new Scanner(System.in);
        String T = in.nextLine();
        in.close();
        FileReader fr = new FileReader("lib/alphabet.txt");
        BufferedReader br = new BufferedReader(fr);
        for (int i = 0; i < H; i++) {
            String ROW = br.readLine();
            int k = L;
            for (int j = 0; j < ROW.length(); j+=L)
            {
                String substring = ROW.substring(j,k);
                betuk.add(substring);
                k += L;

            }
        }
        br.close();

        String[][] betukmatrix = new String[H][28];
        int a = 0;
        int b = 0;
        int c = 0;
            while (a < H)
            {
                while (b < 28)
                {
                    betukmatrix[a][b] = betuk.get(c);
                    b++;
                    c++;
                }
                a++;
                b = 0;
            }
        

        char[] abc = {'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',' ','?'};
        char[] bemenet = T.toUpperCase().toCharArray();
        ArrayList<Integer> indexek = new ArrayList<Integer>();
        int nincs = 0;

        for (int j = 0; j < bemenet.length; j++)
            {
                for (int i = 0; i < abc.length; i++)
                {
                    nincs++;
                    if (abc[i] == bemenet[j])
                    {
                        indexek.add(i);
                        nincs = 0;
                    }
                    else if (nincs == abc.length)
                    {
                        indexek.add(27);
                    }
                    
                }
                nincs = 0;
            }

        String[][] answer = new String[H][indexek.size()]; 
        for (int i = 0; i < H; i++)
        {
            for (int j = 0; j < indexek.size(); j++)
            {
                answer[i][j] = betukmatrix[i][indexek.get(j)];
            }
        }

        for (int i = 0; i < H; i++)
        {
            for (int j = 0; j < indexek.size(); j++)
            {
                System.out.print(answer[i][j]);
            }
            System.out.println();
        }

    }
}
