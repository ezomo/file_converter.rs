f = open("test.txt","w")
text = "\n".join(list(map(str,list(range(100)))))
f.write(text)
f.close()