import os

stream = os.popen('cargo skyline build')
output = stream.read()
output
input("finished!")
