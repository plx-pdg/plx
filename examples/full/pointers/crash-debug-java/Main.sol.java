public class Main {
  public static void main(String[] args) {
    Person p = new Person(args[0], args[1]);
    System.out.println("Hello there !");
    System.out.println("My fullname is " + p.firstname + " " + p.lastname);
  }
}
