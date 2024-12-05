package samples.inheritance.abstractclasswithoutimpl;

public class AbstractClassWithoutInterfaceImplementation {
    public static void main(String[] args) {
        boolean defaultFlag = true;
        Unmapper defaultUnmapper = defaultFlag ? new DefaultUnmapper() : new CustomUnmapper();
        Unmapper customUnmapper = !defaultFlag ? new DefaultUnmapper() : new CustomUnmapper();

        int result = (defaultUnmapper.isSync() ? 0 : 1) + (customUnmapper.isSync() ? 1 : 0);
        System.out.println(result);
    }

}

interface UnmapperProxy {
    boolean isSync();
}

abstract class Unmapper implements UnmapperProxy {
}

class DefaultUnmapper extends Unmapper {
    @Override
    public boolean isSync() {
        return false;
    }
}

class CustomUnmapper extends Unmapper {
    @Override
    public boolean isSync() {
        return true;
    }
}
