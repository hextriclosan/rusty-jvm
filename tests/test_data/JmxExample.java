package samples.management.jmxexample;

import java.lang.management.ClassLoadingMXBean;
import java.lang.management.ManagementFactory;
import java.lang.management.MemoryMXBean;
import java.lang.management.MemoryUsage;
import java.lang.management.OperatingSystemMXBean;
import java.lang.management.RuntimeMXBean;
import java.lang.management.ThreadMXBean;

public class JmxExample {
    public static void main(String[] args) {
        // RuntimeMXBean
        RuntimeMXBean runtime = ManagementFactory.getRuntimeMXBean();
        System.out.println("vm.name=" + runtime.getVmName());
        System.out.println("vm.vendor=" + runtime.getVmVendor());
        System.out.println("uptime.positive=" + (runtime.getUptime() >= 0));
        System.out.println("pid.positive=" + (runtime.getPid() > 0));

        // MemoryMXBean
        MemoryMXBean memory = ManagementFactory.getMemoryMXBean();
        MemoryUsage heap = memory.getHeapMemoryUsage();
        System.out.println("heap.max.positive=" + (heap.getMax() > 0));

        // ThreadMXBean
        ThreadMXBean threads = ManagementFactory.getThreadMXBean();
        System.out.println("thread.count.positive=" + (threads.getThreadCount() > 0));
        System.out.println("daemon.count.nonneg=" + (threads.getDaemonThreadCount() >= 0));

        // ClassLoadingMXBean
        ClassLoadingMXBean cl = ManagementFactory.getClassLoadingMXBean();
        System.out.println("class.loaded.nonneg=" + (cl.getLoadedClassCount() >= 0));
        System.out.println("class.total.nonneg=" + (cl.getTotalLoadedClassCount() >= 0));

        // OperatingSystemMXBean
        OperatingSystemMXBean os = ManagementFactory.getOperatingSystemMXBean();
        System.out.println("avail.procs.positive=" + (os.getAvailableProcessors() > 0));
    }
}
