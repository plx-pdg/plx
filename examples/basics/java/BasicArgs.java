public class BasicArgs {
  public static void main(String[] args) {
    if (args.length < 2)
      System.out.println("Error: missing argument firstname and legs number");
    else
      System.out.println("The dog is " + args[0] + " and has " + args[1] + " legs\n");
  }
}
