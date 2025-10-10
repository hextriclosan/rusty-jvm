package samples.module.inspectmodule;

import org.xml.sax.XMLReader;

import java.lang.module.ModuleDescriptor;
import java.util.Set;
import java.util.TreeSet;

public class InspectModule {
    public static void main(String[] args) {
        printModuleInfo(String.class);  // java.base
        printModuleInfo(XMLReader.class);  // java.xml
        printModuleInfo(InspectModule.class);  // this class
    }

    private static void printModuleInfo(Class<?> clazz) {
        System.out.println("Class: " + clazz);
        Module module = clazz.getModule();
        System.out.println("  Module name: " + (module.isNamed() ? module.getName() : "unnamed module"));
        //System.out.println("  classLoader: " + module.getClassLoader()); // todo: unnamed module should return jdk.internal.loader.ClassLoaders$AppClassLoader
        //System.out.println("  packages: " + module.getPackages()); // Native Call Error: Native method jdk/internal/loader/BootLoader:getSystemPackageNames:()[Ljava/lang/String; not found
        printModuleDescriptorInfo(module.getDescriptor());
        System.out.println();
    }

    private static void printModuleDescriptorInfo(ModuleDescriptor descriptor) {
        if (descriptor == null) {
            System.out.println("No ModuleDescriptor");
            return;
        }
        System.out.println("    ModuleDescriptor:");
        System.out.println("      name: " + descriptor.name());
        System.out.println("      modifiers: " + canonicalize(descriptor.modifiers()));
        System.out.println("      requires: " + canonicalize(descriptor.requires()));
        System.out.println("      exports:");
        printExports((Set<ModuleDescriptor.Exports>)canonicalize(descriptor.exports()));
        System.out.println("      opens: " + canonicalize(descriptor.opens()));
        System.out.println("      uses: " + canonicalize(descriptor.uses()));
        System.out.println("      provides: " + canonicalize(descriptor.provides()));
    }

    private static void printExports(Set<ModuleDescriptor.Exports> exports) {
        for (ModuleDescriptor.Exports export : exports) {
            if ("sun.nio.cs".equals(export.source()) || "sun.security.internal.spec".equals(export.source()) || "sun.security.rsa".equals(export.source())) {
                // Skip platform-specific exports
                continue;
            }
            System.out.println("        " + export.source() + " " + canonicalize(export.targets()) + " " + canonicalize(export.modifiers()));
        }
    }

    private static Set<?> canonicalize(Set<?> set) {
        return new TreeSet<>(set);
    }
}
