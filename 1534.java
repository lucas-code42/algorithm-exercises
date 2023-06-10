import java.io.IOException;
import java.util.Scanner;

public class Main {
    
    public static void main(String[] args) throws IOException {
        Scanner leitor = new Scanner(System.in);
    	int N;
        
        while (leitor.hasNext()) {
    		N = leitor.nextInt();
    
    		for (int i = 0; i < N; i++) {
    
    			for (int j = 0; j < N; j++) {
    				if (j == N-i-1) System.out.print(2);
    				else if (i == j) System.out.print(1);
    				else System.out.print(3);
    			}
    
    			System.out.print("\n");
    		}
    	}
    }
	
}