package sun.nio.ch;

import java.io.File;
import java.io.IOException;
import jdk.internal.util.OS;
import java.io.FileDescriptor;

class FileDispatcherImpl extends FileDispatcher {
    private final FileDispatcher delegate;

    public FileDispatcherImpl() {
        this.delegate = createPlatformDispatcher();
    }

    private FileDispatcher createPlatformDispatcher() {
        if (OS.WINDOWS) {
            return new WindowsFileDispatcherImpl();
        } else if (OS.MACOS) {
            return new MacOSXFileDispatcherImpl();
        } else if (OS.LINUX) {
            return new LinuxFileDispatcherImpl();
        } else {
            throw new UnsupportedOperationException("Unsupported OS");
        }
    }

    int setDirectIO(FileDescriptor fd, String path) {
        return delegate.setDirectIO(fd, path);
    }

    long transferFrom(FileDescriptor src, FileDescriptor dst,
                      long position, long count, boolean append) {
        return delegate.transferFrom(src, dst, position, count, append);
    }

    long transferTo(FileDescriptor src, long position, long count,
                    FileDescriptor dst, boolean append) {
        return delegate.transferTo(src, position, count, dst, append);
    }

    int maxDirectTransferSize() {
        return delegate.maxDirectTransferSize();
    }

    int unmap(long address, long length) {
        return delegate.unmap(address, length);
    }

    long map(FileDescriptor fd, int prot, long position, long length,
             boolean isSync)
        throws IOException
    {
        return delegate.map(fd, prot, position, length, isSync);
    }

    long allocationGranularity() {
        return delegate.allocationGranularity();
    }

    boolean canTransferToFromOverlappedMap() {
        return delegate.canTransferToFromOverlappedMap();
    }

    boolean transferToDirectlyNeedsPositionLock() {
        return delegate.transferToDirectlyNeedsPositionLock();
    }

    boolean canTransferToDirectly(java.nio.channels.SelectableChannel sc) {
        return delegate.canTransferToDirectly(sc);
    }

    FileDescriptor duplicateForMapping(FileDescriptor fd) throws IOException {
        return delegate.duplicateForMapping(fd);
    }
    void release(FileDescriptor fd, long pos, long size) throws IOException {
        delegate.release(fd, pos, size);
    }
    int lock(FileDescriptor fd, boolean blocking, long pos, long size,
             boolean shared) throws IOException
    {
        return delegate.lock(fd, blocking, pos, size, shared);
    }
    long size(FileDescriptor fd) throws IOException {
        return delegate.size(fd);
    }
    int truncate(FileDescriptor fd, long size) throws IOException {
        return delegate.truncate(fd, size);
    }
    int force(FileDescriptor fd, boolean metaData) throws IOException {
        return delegate.force(fd, metaData);
    }

    int read(FileDescriptor fd, long address, int len)
        throws IOException
    {
        return delegate.read(fd, address, len);
    }

    int pread(FileDescriptor fd, long address, int len, long position)
        throws IOException
    {
        return delegate.pread(fd, address, len, position);
    }

    long readv(FileDescriptor fd, long address, int len) throws IOException {
        return delegate.readv(fd, address, len);
    }

    int write(FileDescriptor fd, long address, int len) throws IOException {
        return delegate.write(fd, address, len);
    }

    int pwrite(FileDescriptor fd, long address, int len, long position)
        throws IOException
    {
        return delegate.pwrite(fd, address, len, position);
    }

    long writev(FileDescriptor fd, long address, int len) throws IOException {
        return delegate.writev(fd, address, len);
    }

    long seek(FileDescriptor fd, long offset) throws IOException {
        return delegate.seek(fd, offset);
    }
    void close(FileDescriptor fd) throws IOException {
        delegate.close(fd);
    }

    // Delegate other methods similarly...
}
