import java.util.ArrayList;
import java.util.List;

public class DirObj implements TerminalObj {

    private final String name;
    private final List<TerminalObj> children;
    private final DirObj parent;

    public DirObj(String name, DirObj parent) {
        this.name = name;
        this.children = new ArrayList<>();
        this.parent = parent;
    }

    public void addChild(TerminalObj child) {
        children.add(child);
    }

    @Override
    public int getSize() {
        int size = 0;

        for (TerminalObj c : children) {
            size += c.getSize();
        }

        return size;
    }

    @Override
    public DirObj getParent() {
        return parent;
    }

    public List<TerminalObj> getChildren() {
        return children;
    }

    public String getName() {
        return name;
    }
}
