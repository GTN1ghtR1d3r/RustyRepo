import os
os.system("git add --all")
msg = input("provide commit msg: ")
os.system(f"git commit -m {msg}")
os.system("git push")