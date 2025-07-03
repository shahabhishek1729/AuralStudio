def adjoint(a, b, c, d, det):
    a = d / det
    b = -1 * b / det
    c = -1 * c / det
    d = a / det
    inv = [a, b, c, d]
    return inv
    
def determinant(a, b, c, d):
    answer = a * d - b * c
    if answer == 0:
        print("non-invertible!")
        
    else:
        print("invertible!")
        
    
def start(arguments):
    a = 1
    b = 2
    c = 3
    d = 4
    det = determinant(a, b, c, d)
    inv = adjoint(a, b, c, d, det)
    print(inv)
    
if __name__ == '__main__':
	start([])