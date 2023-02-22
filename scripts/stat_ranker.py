import os

i = []
o = []
rank = {}
import os
import tkinter as tk
from tkinter import filedialog

root = tk.Tk()
root.withdraw()
input_file = filedialog.askopenfilename()

def inputter():
  global i
  global input_file
  with open(input_file) as f:
    i = f.readlines()
  f.close()
def ranker(chosen_check, is_print):
  global i
  global o
  global rank
  inputter()
  char = ""
  rank = {}
  for f in i:
    if '<hash40 hash="fighter_kind">' in f:
      char = f.replace('<hash40 hash="fighter_kind">', "")
      char = char.replace('fighter_kind_', "")
      char = char.replace('</hash40>', "")
      char = char.replace('\n', "")
      char = char.replace(' ', "")
    if chosen_check in f and char not in ["koopag", "nana", "miienemyg", "miienemyf", "miienemys"]:
      new_num = f.replace(chosen_check, "")
      new_num = new_num.replace("</float>", "")
      new_num = new_num.replace("</int>", "")
      new_num = new_num.replace(" ", "")
      new_num = float(new_num)
      rank[char] = new_num
  sort_rank = sorted(rank.items(), key=lambda x: x[1], reverse=True)
  if is_print == True:
    print("TOP 10 " + chosen_check)
    for i in range(0,10):
      x = sort_rank[i]
      print(x[0] + " " + str(x[1]))
    print("\n\nBOTTOM 10 " + chosen_check)
    for i in range(1,11):
      x = sort_rank[i*-1]
      print(x[0] + " " + str(x[1]))
    print("AVERAGES")
    total = 0
    median = 0
    for i in range(0, 89):
      x = sort_rank[i]
      total += x[1]
      if i == 44:
        median = x[1]
    mean = total/89
    print("Mean: " + str(mean) )
    print("Median: " + str(median))
  chosen_check = chosen_check.replace("<", "")
  chosen_check = chosen_check.replace(">", "")
  chosen_check = chosen_check.replace("hash=", "")
  chosen_check = chosen_check.replace("float", "")
  chosen_check = chosen_check.replace('"', "")
  chosen_check = chosen_check.replace(' ', "")
  if chosen_check == "ground_brake":
    chosen_check = "traction"
  if chosen_check == "air_speed_y_stable":
    chosen_check = "fallspeed"
  if chosen_check == "air_accel_y":
    chosen_check = "gravity"
  if chosen_check == "air_speed_x_stable":
    chosen_check = "air_speed"
  if not os.path.exists(".stats"):
    os.makedirs(".stats")
  if os.path.exists(f".stats/rank {chosen_check}.txt"):
    os.remove(f".stats/rank {chosen_check}.txt")
  with open(f".stats/rank {chosen_check}.txt", 'a') as f:
      f.write(chosen_check + " Ranking:\n")
      for i in range(0,len(sort_rank)):
        x = sort_rank[i]
        f.write(str(i+1) + ". " + x[0] + " " + str(x[1]) + "\n")
  f.close()
  i = []
  o = []


inputter()
x = input("What are you checking?")
if x == "dashspeed":
  x = '<float hash="dash_speed">'
elif x == "walkspeed":
  x = '<float hash="walk_speed_max">'
elif x == "runspeed":
  x = '<float hash="run_speed_max">'
elif x == "shorthop":
  x = '<float hash="mini_jump_y">'
elif x == "fullhop":
  x = '<float hash="jump_y">'
elif x == "doublejump":
  x = '<float hash="jump_aerial_y">'
elif x == "fallspeed":
  x = '<float hash="air_speed_y_stable">'
elif x == "gravity":
  x = '<float hash="air_accel_y">'
elif x == "traction":
  x = '<float hash="ground_brake">'
elif x == "airspeed":
  x = '<float hash="air_speed_x_stable">'
elif x == "air accel add":
  x = '<float hash="air_accel_x_add">'
elif x == "air accel mul":
  x = '<float hash="air_accel_x_mul">'
elif x == "weight":
  x = '<float hash="weight">'
elif x == "jump_speed_x":
  x = '<float hash="jump_speed_x">'
elif x == "jump_speed_x":
  x = '<float hash="jump_speed_x">'
elif x == "jump_speed_x_mul":
  x = '<float hash="jump_speed_x_mul">'
elif x == "jump_speed_x_max":
  x = '<float hash="jump_speed_x_max">'
if x == 'rank_all':
  ranker('<float hash="dash_speed">', False)
  ranker('<float hash="run_speed_max">', False)
  ranker('<float hash="walk_speed_max">', False)
  ranker('<float hash="mini_jump_y">', False)
  ranker('<float hash="jump_y">', False)
  ranker('<float hash="jump_aerial_y">', False)
  ranker('<float hash="air_speed_y_stable">', False)
  ranker('<float hash="air_accel_y">', False)
  ranker('<float hash="ground_brake">', False)
  ranker('<float hash="air_speed_x_stable">', False)
  ranker('<float hash="air_accel_x_add">', False)
  ranker('<float hash="air_accel_x_mul">', False)
  ranker('<float hash="weight">', False)
  ranker('<float hash="jump_speed_x">', False)
  ranker('<float hash="jump_speed_x_mul">', False)
  ranker('<float hash="jump_speed_x_max">', False)
else:
  ranker(x, True)
  input("")