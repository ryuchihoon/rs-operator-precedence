# rs-operator-precedence
Toy rust implementation of operator precedence

input spec:
 - one character identifiers
 - +, -, *, / binary operator

example input:
```
a+b*c/d-e-f*g*h
```

rpn form result:
```
abc*d/+e-fg*h*-
```
