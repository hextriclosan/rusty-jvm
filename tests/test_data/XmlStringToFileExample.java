package samples.io.xmlstringtofileexample;

import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.NodeList;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import java.io.File;
import java.io.FileWriter;

public class XmlStringToFileExample {
    public static void main(String[] args) throws Exception {
        // 1. Create temp file
        File tempFile = File.createTempFile("person", ".xml");
        tempFile.deleteOnExit();

        // 2. XML content as string
        String xmlContent = """
                <?xml version="1.0"?>
                <person>
                    <name>John</name>
                    <age>30</age>
                </person>
                """;

        // 3. Write string to file
        try (FileWriter writer = new FileWriter(tempFile)) {
            writer.write(xmlContent);
        }

        // 4. Parse XML file
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        Document doc = builder.parse(tempFile);
        doc.getDocumentElement().normalize();

        // 5. Read values
        Element root = doc.getDocumentElement();
        System.out.println("Root element: " + root.getNodeName());

        NodeList names = doc.getElementsByTagName("name");
        NodeList ages = doc.getElementsByTagName("age");

        System.out.println("Name: " + names.item(0).getTextContent());
        System.out.println("Age: " + ages.item(0).getTextContent());
    }
}
