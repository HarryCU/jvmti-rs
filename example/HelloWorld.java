class HelloWorld {
    public static void main(String[] args) throws Exception {

        // System.in.read();
        System.out.println("jvmti-rs >> Hello!");

        System.out.println("jvmti-rs >> Try jvmti exception");

        try {
            throwException();
        } catch(Exception e) {
            System.out.println("jvmti-rs >> " + e.getMessage() + "!");
        }

        System.out.println("jvmti-rs >> Byte!");
    }

    public static void throwException() throws Exception {
        throw new Exception("123");
    }
}
