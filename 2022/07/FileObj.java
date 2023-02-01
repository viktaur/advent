public class FileObj implements TerminalObj {

    private final String name;
    private final int size;
    private final DirObj parent;


    public FileObj(String name, int size, DirObj parent) {
        this.name = name;
        this.size = size;
        this.parent = parent;
    }

    @Override
    public int getSize() {
        return size;
    }

    @Override
    public DirObj getParent() {
        return parent;
    }
}
