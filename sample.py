def invert_matrix(a, b, c, d):
    def adjoint(a, b, c, d):
        return [d, -b, -c, a]

    def inverse_determinant(a, b, c, d):
        det = a * d - b * c
        return 1/det

    inv_det = inverse_determinant(a, b, c, d)
    adj = adjoint(a, b, c, d)

    for i in range(3):
        adj[i] *= inv_det


def print_matrix(matrix):
    print(str(matrix[0]) + " " + str(matrix[1]))
    print(str(matrix[2]) + " " + str(matrix[3]))

def start():
    matrix = [1, 2, 3, 4]
    print_matrix(matrix)
    print("------------")
    print_matrix(invert_matrix(matrix[0], matrix[1], matrix[2], matrix[3]))

if __name__ == "__main__":
    start()

