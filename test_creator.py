f = open("test.txt","w")
text = "\n".join(list(map(str,list(range(10000)))))
f.write(text)
f.close()