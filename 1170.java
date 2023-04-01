package uri_1170;

import java.util.Locale;
import java.util.Scanner;

public class Main {

    public static void main(String[] args) {
        Locale.setDefault(Locale.US);
        Scanner sc = new Scanner(System.in);

        int rounds = sc.nextInt();

        for (int i = 0; i < rounds; i++) {
            int count = 0;
            double calc = sc.nextDouble();

            do {
                calc = calc / 2;
                count++;
            } while (!(calc < 1.0));

            System.out.println(count +  dias);
        }

        sc.close();
    }
}

