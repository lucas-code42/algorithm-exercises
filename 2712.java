import java.util.HashMap;
import java.util.Map;
import java.util.Scanner;

public class Main {
	public static void main(String[] args) {
		Scanner sc = new Scanner(System.in);

		int runTimes = sc.nextInt();
		while (runTimes > 0) {
			String licensePlate = sc.next();
			
			boolean matchFound = licensePlate.matches("[A-Z][A-Z][A-Z]-[0-9][0-9][0-9][0-9]");
			if (matchFound) {
				String lastChar = (licensePlate.substring(licensePlate.length() - 1));
				String rotationDay = getRotationDay(lastChar);
				System.out.println(rotationDay);
			} else {
				System.out.println("FAILURE");
			}
			runTimes--;
		}

		sc.close();
	}

	public static String getRotationDay(String letter) {
		Map<String, String> rotationDay = new HashMap<>();

		rotationDay.put("1", "MONDAY");
		rotationDay.put("2", "MONDAY");
		rotationDay.put("3", "TUESDAY");
		rotationDay.put("4", "TUESDAY");
		rotationDay.put("5", "WEDNESDAY");
		rotationDay.put("6", "WEDNESDAY");
		rotationDay.put("7", "THURSDAY");
		rotationDay.put("8", "THURSDAY");
		rotationDay.put("9", "FRIDAY");
		rotationDay.put("0", "FRIDAY");

		return rotationDay.get(letter);

	}
}