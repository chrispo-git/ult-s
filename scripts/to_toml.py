import os
import xml.etree.ElementTree as ET

def convert_prcxml_to_toml(file_path, output_name):
    # Use a parser that specifically preserves XML comments
    parser = ET.XMLParser(target=ET.TreeBuilder(insert_comments=True))
    
    try:
        tree = ET.parse(file_path, parser=parser)
        root = tree.getroot()
    except Exception as e:
        print(f"Error parsing file: {e}")
        return

    output = []
    current_kind = "unknown"

    # .iter() finds every tag and comment, no matter how deep they are nested
    for node in root.iter():
        
        # Check if the node is a comment (e.g., )
        if callable(node.tag) and "comment" in str(node.tag).lower():
            raw_comment = node.text.strip()
            # Clean up the comment for the 'kinds' field (e.g., "YOUNG LINK" -> "young_link")
            current_kind = raw_comment.lower().replace(" ", "_").replace("/", "").replace("&", "n")
            output.append(f"\n# {raw_comment}")
            continue

        # Only process data tags (float, int, bool)
        if node.tag in ['float', 'int', 'bool']:
            param_name = node.attrib.get('hash', 'unknown')
            raw_value = node.text.strip() if node.text else "0"

            if node.tag == "bool":
                header = "[[param_int]]"
                # Boolean logic: true -> 1, false -> 0
                val = 1 if raw_value.lower() == "true" else 0
            else:
                header = f"[[param_{node.tag}]]"
                try:
                    # Force float values to have .0 (e.g., 9 becomes 9.0)
                    val = float(raw_value) if node.tag == "float" else int(raw_value)
                except ValueError:
                    val = raw_value

            entry = (
                f"{header}\n"
                f'param = "{param_name}"\n'
                f'subparam = ""\n'
                f'value = {val}\n'
                f'kinds = ["{current_kind}"]\n'
                f'slots = [0,1,2,3,4,5,6,7]\n'
            )
            output.append(entry)

    # Write the results
    if output:
        with open(output_name, "w") as f:
            f.write("\n".join(output))
        print(f"Success! Created {output_name}")
    else:
        print("No data found to convert.")

# --- RUN THE CONVERTER ---
# Ensure this path matches your folder structure
input_file = '../romfs/fighter/common/param/fighter_param.prcxml'
convert_prcxml_to_toml(input_file, "out.toml")