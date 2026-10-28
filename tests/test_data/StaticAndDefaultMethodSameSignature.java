package samples.inheritance.staticanddefaultmethodsamesignature;

public class StaticAndDefaultMethodSameSignature {
    public static void main(String[] args) {
        Impl impl = new Impl();
        System.out.println(impl.describe());

        WithDefault withDefault = impl;
        System.out.println(withDefault.describe());

        SubImpl subImpl = new SubImpl();
        System.out.println(subImpl.describe());

        System.out.println(WithStatic.describe());
        System.out.println(SubImpl.describeStatically());
    }
}

interface WithStatic {
    static String describe() {
        return "static WithStatic.describe";
    }
}

interface WithDefault {
    default String describe() {
        return "default WithDefault.describe";
    }
}

// Static interface methods are not inherited, so describe() resolves to the default method
class Impl implements WithStatic, WithDefault {
    static String describeStatically() {
        return "static Impl.describeStatically";
    }
}

class SubImpl extends Impl {
}
