import xml.etree.ElementTree as ET

def main():
    tree = ET.parse('bookmark_chrome.html')
    root = tree.getroot()

    for child in root:
        print(child.tag, child.attrib)

main()
