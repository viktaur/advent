import java.io.*;
import java.util.Comparator;
import java.util.PriorityQueue;

import static java.lang.Character.isDigit;

public class Main {

    private static final DirObj root = new DirObj("/", null); // root node, it has no parent
    private static int size1 = 0; // variable that will be used as output for part_one.

    private static final int TOTAL_DISK_SPACE = 70000000;
    private static final int UNUSED_SPACE_NEEDED = 30000000;

    private static final Comparator<DirObj> comparator = Comparator.comparingInt(DirObj::getSize);

    private static final PriorityQueue<DirObj> validDirs = new PriorityQueue<>(comparator);

    public static void main(String[] args) {
        try {
            buildTreeFromRoot();

            partOne();
            System.out.println(size1);

            partTwo();
            System.out.println(partTwo());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void partOne() throws IOException {
        addSize(root);
    }

    public static int partTwo() throws IOException {
        int unusedSpace = TOTAL_DISK_SPACE - root.getSize();
        validDirs.add(root);

        findValidDirs(root, unusedSpace);

        assert validDirs.peek() != null;
        return validDirs.peek().getSize();
    }

    /**
     * addSize iterates over every child of current.
     * If it's a DirObj we're going to apply the function again and if it's also less than 100000 we're going to add its size to size1
     * @param current is the current directory
     */
    public static void addSize(DirObj current) {
        for (TerminalObj c : current.getChildren()) {
            if (c instanceof DirObj) {
                addSize((DirObj) c);

                if (c.getSize() <= 100000) {
                    size1 += c.getSize();
                }
            }
        }
    }

    /**
     * Finds child by given name in given dir
     * @param dir given dir
     * @param name given name
     * @return DirObj child
     */
    public static DirObj findChildByName(DirObj dir, String name) {
        for (TerminalObj c : dir.getChildren()) {
            if (c instanceof DirObj && ((DirObj) c).getName().equals(name)) {
                return ((DirObj) c);
            }
        }

        return null;
    }

    public static void buildTreeFromRoot() throws IOException {
        DirObj currentNode = root;

        BufferedReader reader = new BufferedReader(new FileReader("src/main/resources/input.txt"));
        String line;

        // build tree
        while ((line = reader.readLine()) != null) {
            if (line.startsWith("$ ")) { // handle input
                if (line.startsWith("$ cd /")) {
                    currentNode = root;
                } else if (line.startsWith("$ cd ..")) {
                    if (currentNode != root) {
                        assert currentNode != null;
                        currentNode = currentNode.getParent();
                    }
                } else if (line.startsWith("$ cd ")) {
                    String newDirName = line.substring(5);
                    assert currentNode != null;
                    currentNode = findChildByName(currentNode, newDirName);
                }
            } else { // handle output
                TerminalObj child;

                if (isDigit(line.charAt(0))) { // is a file
                    int fSize = Integer.parseInt(line.split(" ")[0]);
                    String fName = line.split(" ")[1];

                    child = new FileObj(fName, fSize, currentNode);
                } else { // is a dir
                    String dName = line.substring(4);
                    child = new DirObj(dName, currentNode);
                }

                assert currentNode != null;
                currentNode.addChild(child);
            }

        }
    }

    public static void findValidDirs(DirObj currentDir, int unusedSpace) {

        for (TerminalObj c : currentDir.getChildren()) {
            if (c instanceof DirObj) {
                // if removing the dir can make the unused space bigger than the needed, we'll dig further
                // otherwise, there is no point on keeping exploring
                if (unusedSpace + c.getSize() >= UNUSED_SPACE_NEEDED) {
                    validDirs.add((DirObj) c); // add child to the validDirs pq
                    findValidDirs((DirObj) c, unusedSpace); // explore child subtree
                }
            }
        }
    }
}
