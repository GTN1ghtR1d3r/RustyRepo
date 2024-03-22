import os
import sys
if(len(sys.argv)>1):
    if(sys.argv[1] == "-q"):
        msg = "quick update"
if(len(sys.argv) == 1):
    msg = input("provide commit msg: ")

#changed comments
os.system("git add --all")
os.system(f"git commit -m \"{msg}\"")
os.system("git push")