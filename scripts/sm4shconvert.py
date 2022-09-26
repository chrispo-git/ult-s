import os
import time

inp = []
replace = []
import os
import tkinter as tk
from tkinter import filedialog

root = tk.Tk()
root.withdraw()
input_file = filedialog.askopenfilename()

def inputter():
  global inp
  global input_file
  with open(input_file) as f:
    inp = f.readlines()
  f.close()

def replacer_open():
  global replace
  global input_file
  with open("replace.txt") as f:
    repl = f.readlines()
    for i in repl:
        newline = i.replace("\n", "")
        replace.append(newline.split(", "))
  f.close()


def converter():
  global i
  global replace
  inputter()
  replacer_open()
  if os.path.exists('output.smd'):
    os.remove('output.smd')
  with open(f"output.smd", 'a') as f:
      for i in inp:
        new = i
        for w in replace:
            new = new.replace(w[0], w[1])
        f.write(new)
  f.close()
  i = []
  replace = []


converter()
print("Finished!")
time.sleep(4)

