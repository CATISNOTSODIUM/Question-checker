import sys
import yaml
from pathlib import Path
import os 
import json
import shutil
#current directory
try:
    file_name = sys.argv[1]
except:
    print("Insert your file name.")
    exit()
    
if (file_name=="--h" or file_name=="--help" ):
    print("python3 gen_dir.py [file_name (yml)] will generate placeholder files and directories based on the schema")
    exit()


root_dir =  os.path.join(os.getcwd(), "../") 
target_path = os.path.join(root_dir, "assets","materials") 
file_path = os.path.join(root_dir, "assets","schema",str(file_name)) #will use as an argument

#read flag
try:
    flag = sys.argv[2]
    is_delete = input("Do you want to clear all files and folders in the assets/material directory? [y/n] ")
    if (is_delete.lower() in ["y", "yes"]):
        shutil.rmtree(target_path)
        os.mkdir(target_path)
    else:
        exit()
    
except:
    pass

#read yml
try:    
    directory_tree = yaml.safe_load(Path(file_path).read_text())
except:
    print("Cannot find ", file_name)
    exit()

def create_structure(parent_dir, structure):
    for key, value in structure.items():
        current_dir = os.path.join(parent_dir, str(key))
        os.makedirs(current_dir, exist_ok=True)
        if isinstance(value, dict):
            create_structure(current_dir, value)
        else:
            for file_name in value:
                file_path = os.path.join(current_dir, file_name)
                open(file_path, 'a').close()

# Create the directory structure and placeholder files
create_structure(target_path, directory_tree)
print("Complete!")