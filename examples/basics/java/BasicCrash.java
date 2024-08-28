class Person {
  public String firstname;
  public String lastname;
}

public class BasicCrash {
  public static void main(String[] args) {
    Person p = null;
    System.out.println("Hello there !");
    System.out.println("My name is " + p.firstname);
  }
}
