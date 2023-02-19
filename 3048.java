package uri_3048;


import java.util.ArrayList;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int[] array = new int[n];

        for (int i = 0; i < n; i++) {
            array[i] = sc.nextInt();
        }

        System.out.println(getTarget(array));


        sc.close();

    }

    public static int getTarget(int[] data) {
        ArrayList<Integer> array = new ArrayList<>();

        array.add(data[0]);
        int tmp = data[0];
        for (int i = 0; i < data.length; i++) {

            if (data[i] != tmp) {
                array.add(data[i]);
                tmp = data[i];
            }

        }

        return array.size();
    }
}
