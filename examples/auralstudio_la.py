def invert(m):
    def determinant(m):
        x = m[0] * m[3]
        y = m[1] * m[2]
        return x - y
        
    def adjoint(m):
        result = [m[3], 0 - m[1], 0 - m[2], m[0]]
        return result
        
    d = determinant(m)
    a = adjoint(m)
    iterator = range(4)
    for i in iterator:
        m[i] = 1 / d * a[i]
        return m
        
    
def g(x):
    print(x + 1)
    if x == 3:
        print(x)
        
    else:
        print(y)
        
    
def start(args):
    matrix = [1, 2, 3, 4]
    result = invert(matrix)
    print(result)
    

if __name__ == '__main__':
	start([])
